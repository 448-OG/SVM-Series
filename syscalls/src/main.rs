use solana_bpf_loader_program::syscalls::SyscallLog;
use solana_compute_budget::compute_budget::ComputeBudget;
use solana_program_runtime::{
    // `InvokeContext` is used to configure the entire exeuction pipeline
    invoke_context::InvokeContext,
    solana_sbpf::{
        program::BuiltinProgram,
        // Rename from `Config` to `SbpfVmConfig` to avoid conflicts with other deps with structs called `Config`
        vm::Config as SbpfVmConfig,
    },
};

fn main() {
    // Initialize `ComputeBudget` struct with default parameters.
    // These parameters include:
    //    - compute unit limit (defaults to 1.4 million CUs)
    //    - Maximum nesting of instructions that can happen during a transaction (defaults to 5)
    //    - Maximum cross-program invocation and instructions per transaction (defaults to 64)
    //    - Maximum number of slices hashed per syscall (defaults to 20,000)
    //    - Maximum SBF to BPF call nesting that can happen within a program (defaults to 64 SBF to BPF to BPF calls)
    //    - Size of a stack frame in bytes matching the size specified in the LLVM SBF backend (defaults to 4096)
    //    - Maximum cross-program invocation instruction size (default to IPv6 Min MTU size of 1280)
    //    - Length of the heap memory region used for program heap
    //      (defaults to solana_program_entrypoint::HEAP_LENGTH of 32768 = 1024 * 32)
    //    - Loads the default execution costs defined by `solana_program_runtime::execution_budget::SVMTransactionExecutionCost` struct
    //      which is defines cost parameters for operations like logging, CPI, signature verification and more.
    let compute_budget_config = ComputeBudget::default();

    // Defining syscalls requires some configuration using `solana_sbpf::vm::Config`.
    // This configures maximum call depth, stack frame size, copying read only data, tracing instructions,
    // allowed SBF versions, sanitizing user provided values and many more
    let vm_config = SbpfVmConfig {
        // configure maximum call depth using that of our compute budget `compute_budget_config`
        max_call_depth: compute_budget_config.max_call_depth,
        // configure stack frame size using that of our compute budget `compute_budget_config`
        stack_frame_size: compute_budget_config.stack_frame_size,
        // Enable instruction tracing
        enable_instruction_tracing: true,
        // enable symbol and section labels for BPF (disabled by default)
        enable_symbol_and_section_labels: true,
        // Reject ELF files containing issues that the verifier did not catch before (up to v0.2.21). Disabled by default
        reject_broken_elfs: true,
        // Avoid copying read only sections when possible. Enabled by default
        optimize_rodata: false,
        // Use all other default parameters
        ..Default::default()
    };

    // Next we specify the syscalls the compiled program can call during execution
    // using solana_sbpf::program::BuiltinProgram.
    let mut loader: BuiltinProgram<InvokeContext<'_>> = BuiltinProgram::new_loader(vm_config);
    loader
        .register_function("sol_log_", SyscallLog::vm)
        .expect("Registration of the `sol_log_` syscall failed.")
}

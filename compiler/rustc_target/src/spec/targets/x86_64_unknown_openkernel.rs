use crate::spec::{
    Arch, Cc, CodeModel, Env, LinkerFlavor, Lld, Os, PanicStrategy, RelroLevel, Target,
    TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    let opts = TargetOptions {
        os: Os::OpenKernel,
        vendor: "unknown".into(),
        env: Env::Unspecified,
        cpu: "x86-64".into(),
        max_atomic_width: Some(64),
        plt_by_default: false,
        position_independent_executables: false,
        static_position_independent_executables: false,
        relro_level: RelroLevel::Off,
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::No),
        linker: Some("ld".into()),
        executables: true,
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        code_model: Some(CodeModel::Large),
        ..Default::default()
    };
    Target {
        llvm_target: "x86_64-unknown-none-elf".into(),
        metadata: TargetMetadata {
            description: Some("x86_64 Open Kernel userspace".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: Arch::X86_64,
        options: opts,
    }
}

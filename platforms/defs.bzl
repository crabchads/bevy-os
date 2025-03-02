load("@prelude//:build_mode.bzl", "BuildModeInfo")

def _platforms(ctx):
    constraints = dict()
    constraints.update(ctx.attrs.cpu_configuration[ConfigurationInfo].constraints)
    constraints.update(ctx.attrs.os_configuration[ConfigurationInfo].constraints)
    configuration = ConfigurationInfo(
        constraints = constraints,
        values = {},
    )

    name = ctx.label.raw_target()
    platform = ExecutionPlatformInfo(
        label = ctx.label.raw_target(),
        configuration = configuration,
        executor_config = CommandExecutorConfig(
            local_enabled = True,
            remote_enabled = False,
            use_limited_hybrid = False,
        ),
    )

    return [
        DefaultInfo(),
        ExecutionPlatformRegistrationInfo(platforms = [platform]),
        configuration,
        PlatformInfo(label = str(name), configuration = configuration),
    ]

def _action_keys(ctx):
    return [
        DefaultInfo(),
        BuildModeInfo(cell = ctx.attrs.cell, mode = ctx.attrs.mode),
    ]

platforms = rule(
    attrs = {
        "cpu_configuration": attrs.dep(providers = [ConfigurationInfo]),
        "os_configuration": attrs.dep(providers = [ConfigurationInfo]),
    }, 
    impl = _platforms
)

action_keys = rule(
     attrs = {
        "cell": attrs.string(),
        "mode": attrs.string(),
     },
     impl = _action_keys
)

{ pkgs, ... }:

{
  packages = with pkgs; [
    git
    tilt
    podman
    docker
    k9s
    kubectl
    kind
    kyverno-chainsaw
    kustomize
    cargo-llvm-cov
  ];

  languages.rust.enable = true;

  env.LLVM_COV = "${pkgs.rustc.llvm}/bin/llvm-cov";
  env.LLVM_PROFDATA = "${pkgs.rustc.llvm}/bin/llvm-profdata";

  # env.KIND_EXPERIMENTAL_PROVIDER = "podman";
}

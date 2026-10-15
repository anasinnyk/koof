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
  ];

  languages.rust.enable = true;

  # env.KIND_EXPERIMENTAL_PROVIDER = "podman";
}

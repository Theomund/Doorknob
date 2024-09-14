FROM fedora:40
RUN dnf install -y \
    bash-completion-2.11 \
    dnf-plugins-core-4.9.0 \
    gcc-14.2.1 \
    git-2.46.0 \
    gdb-14.2 \
    make-4.4.1 \
    && dnf clean all
RUN dnf copr enable -y mczernek/vale \
    && dnf install -y vale-3.7.1 \
    && dnf clean all
RUN useradd -m developer
USER developer
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="$PATH:$HOME/.cargo/bin"

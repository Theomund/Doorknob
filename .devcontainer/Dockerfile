# Doorknob - Artificial intelligence program written in Rust.
# Copyright (C) 2024 Theomund
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

FROM fedora:40
RUN dnf install -y \
    bash-completion-2.11 \
    dnf-plugins-core-4.9.0 \
    gcc-14.2.1 \
    git-2.46.0 \
    gdb-14.2 \
    make-4.4.1 \
    opus-devel-1.5.1 \
    && dnf clean all
RUN dnf copr enable -y mczernek/vale \
    && dnf install -y vale-3.7.1 \
    && dnf clean all
RUN useradd -m developer
USER developer
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="$PATH:$HOME/.cargo/bin"

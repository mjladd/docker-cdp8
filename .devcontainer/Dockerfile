# Devcontainer for CDP8 development with zsh + oh-my-zsh
FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

ENV DEBIAN_FRONTEND=noninteractive \
    ZSH_DISABLE_COMPFIX=true

# Core build deps + audio libs + zsh
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    pkg-config \
    libasound2-dev \
    libjack-jackd2-dev \
    portaudio19-dev \
    zsh \
    curl \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Ensure default user is zsh
ARG USER=vscode
RUN usermod -s /usr/bin/zsh ${USER}

## Install Oh My Zsh for vscode user + plugins (git + syntax highlighting) (idempotent)
RUN if [ ! -d "/home/${USER}/.oh-my-zsh" ]; then \
            sudo -u ${USER} -H sh -c "curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh | sh -s -- --unattended"; \
        fi \
 && if [ ! -d "/home/${USER}/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting" ]; then \
            sudo -u ${USER} -H git clone https://github.com/zsh-users/zsh-syntax-highlighting.git /home/${USER}/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting; \
        fi \
 && sudo -u ${USER} -H sh -c "if grep -q '^plugins=' ~/.zshrc; then sed -i 's/^plugins=.*/plugins=(git zsh-syntax-highlighting)/' ~/.zshrc; else echo 'plugins=(git zsh-syntax-highlighting)' >> ~/.zshrc; fi" \
 && sudo -u ${USER} -H sh -c "if grep -q '^ZSH_THEME=' ~/.zshrc; then sed -i 's/^ZSH_THEME=.*/ZSH_THEME=\"robbyrussell\"/' ~/.zshrc; else echo 'ZSH_THEME=\"robbyrussell\"' >> ~/.zshrc; fi" \
 && sudo -u ${USER} -H sh -c "grep -q 'zsh-syntax-highlighting.zsh' ~/.zshrc || echo 'source /home/${USER}/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh' >> ~/.zshrc"

WORKDIR /workspaces

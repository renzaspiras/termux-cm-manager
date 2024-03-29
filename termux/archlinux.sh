proot-distro install archlinux
clear
proot-distro login archlinux -- pacman -S neofetch --noconfirm
proot-distro login archlinux -- pacman -S zsh --noconfirm
proot-distro login archlinux -- pacman -S doas --noconfirm
proot-distro login archlinux -- pacman -S neovim --noconfirm
proot-distro login archlinux -- pacman -S wget --noconfirm

proot-distro login archlinux -- wget https://raw.githubusercontent.com/renzaspiras/arch-termux/main/termux/.zshrc ~/.zshrc

proot-distro login archlinux -- chsh -s /bin/zsh
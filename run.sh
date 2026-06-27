#!/bin/bash
# export XDG_RUNTIME_DIR="/run/user/$(id -u)"
echo $(pwd)
./lctrl_rust --get-screen-size
# LD_LIBRARY_PATH=$(pwd) ./lctrl_rust --get-screen-size

echo -e "[Unit]
Description=Lctrl
After=display-manager.service
Requires=display-manager.service

[Service]
Type=simple
ExecStart=$(pwd)/lctrl_rust --run_as_app
Environment=\"XDG_RUNTIME_DIR=/tmp\"
#Environment=\"XDG_RUNTIME_DIR=/tmp\" \"LD_LIBRARY_PATH=$(pwd)\"
WorkingDirectory=$(pwd)

[Install]
WantedBy=graphical.target
" | sudo tee /etc/systemd/system/lctrl.service > /dev/null

sudo systemctl daemon-reload
sudo systemctl enable lctrl
sudo systemctl restart lctrl
sudo systemctl status lctrl

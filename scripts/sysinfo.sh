#!/bin/bash
# Static system info for eww dashboard - outputs JSON

hostname=$(hostname)
os=$(grep PRETTY_NAME /etc/os-release | cut -d= -f2 | tr -d '"')
kernel=$(uname -r)
cpu_model=$(lscpu | grep 'Model name' | cut -d: -f2 | xargs)
cpu_cores=$(nproc)
gpu_model=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null || echo "Unknown")
gpu_vram_total=$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits 2>/dev/null || echo "0")
ram_total=$(free -g | awk '/Mem:/ {print $2}')

cat <<EOF
{
  "hostname": "$hostname",
  "os": "$os",
  "kernel": "$kernel",
  "cpu_model": "$cpu_model",
  "cpu_cores": "$cpu_cores",
  "gpu_model": "$gpu_model",
  "gpu_vram_total": "$gpu_vram_total",
  "ram_total": "$ram_total"
}
EOF

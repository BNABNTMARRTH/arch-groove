#!/bin/bash
VM_IP="${GROOVECORE_VM_IP:-192.168.64.27}"
exec ssh -o StrictHostKeyChecking=no -T "root@${VM_IP}" /usr/local/bin/groovecore-mcp

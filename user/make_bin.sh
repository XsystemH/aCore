#!/bin/bash

# 固定目录路径
DIRECTORY="target/riscv64gc-unknown-none-elf/release"

echo "开始处理目录: $DIRECTORY 中的可执行文件"

# 计数器
count=0

# 遍历指定目录中的文件（非递归）
for file in "$DIRECTORY"/*; do
    # 检查是否为文件且无后缀
    if [ -f "$file" ] && [[ "$file" != *.* ]] && [[ "$file" != .* ]]; then
        # 生成输出文件名
        output_file="${file}.bin"

        # 执行rust-objcopy命令
        echo "处理: $(basename "$file")"
        rust-objcopy --strip-all "$file" -O binary "$output_file"

        if [ $? -eq 0 ]; then
            echo "✓ 已转换: $(basename "$file") -> $(basename "$output_file")"
            ((count++))
        else
            echo "✗ 转换失败: $(basename "$file")"
        fi
    fi
done

echo "完成: 共处理了 $count 个文件"
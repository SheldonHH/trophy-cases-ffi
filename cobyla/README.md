```bash
问题是因为找不到 x86_64-linux-gnu-gcc 和 x86_64-linux-gnu-g++ 编译器。要解决这个问题，可以尝试以下几个步骤：

安装必要的编译器工具链：
确保系统上安装了 gcc 和 g++ 编译器。可以通过以下命令安装：



sudo apt-get update
sudo apt-get install build-essential
安装交叉编译工具链：
如果你在一个不同架构（如 ARM）上构建 x86_64 目标，需要安装相应的交叉编译工具链：


sudo apt-get install gcc-multilib g++-multilib -y
sudo apt-get install gcc-x86-64-linux-gnu g++-x86-64-linux-gnu -y
设置环境变量：
确保编译器在 PATH 中，可以在 .bashrc 或 .zshrc 文件中添加如下内容：


export PATH=$PATH:/usr/bin/x86_64-linux-gnu-gcc
export PATH=$PATH:/usr/bin/x86_64-linux-gnu-g++
# 设置 CMake 编译器路径：
# 在执行 cargo build 命令之前，设置 CMake 的编译器路径：


export CC=/usr/bin/x86_64-linux-gnu-gcc
export CXX=/usr/bin/x86_64-linux-gnu-g++

```
## 重写命令
```bash
RUSTFLAGS=-Zsanitizer=address cargo build -Zbuild-std --target aarch64-unknown-linux-gnu
RUSTFLAGS=-Zsanitizer=thread cargo build -Zbuild-std --target aarch64-unknown-linux-gnu
RUSTFLAGS=-Zsanitizer=leak cargo build -Zbuild-std --target aarch64-unknown-linux-gnu

```

C++ exception,在Rust中怎么处理，直接panic？？
什么C++特征rust难以处理，Rust调用C程序，C++ throw exception直接panic
big endianness
指针传递

讨论：extension




# cobyla

This a Rust wrapper for COBYLA optimizer (COBYLA stands for Constrained Optimization BY Linear Approximations).

COBYLA an algorithm for minimizing a function of many variables. The method is derivatives free (only the function values are needed) 
and take into account constraints on the variables. The algorithm is described in:

  > M.J.D. Powell, "A direct search optimization method that models the objective and constraint functions by linear interpolation," in 
  > Advances in Optimization and Numerical Analysis Mathematics and Its Applications, vol. 275 (eds. Susana Gomez and Jean-Pierre Hennart), 
  > Kluwer Academic Publishers, pp. 51-67 (1994).

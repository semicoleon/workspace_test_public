// RustTestStatic.cpp : Defines the functions for the static library.
//

#include "pch.h"
#include "framework.h"
#include "RustTestStatic.h"
#include <cstdio>


size_t print_int_val(int value)
{
    auto num_printed = printf("Value is: %d\n", value);
    return num_printed;
}

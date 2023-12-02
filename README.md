# Register Generator

[Documentation](DOCS.md) on how to use this solution is available in DOCS.md. This README will cover the  current solutions, their problems, the need for a better solution, and cover how my solution fixes most of these weaknesses.

## Contents
<!--ts-->
  * [Description](#description)
  * [Use Case](#use-case)
  * [Method 1 Bit Shifting and Masking](#method-1-bit-shifting-and-masking)
  * [Method 2 Bit Fields](#method-2-bit-fields)
  * [My Solution](#my-solution)
  * [Weaknesses](#weaknesses)
  * [Planned Future Work](#planned-future-work)
<!--te-->

## Description
As a systems software engineer, I am often working with registers. These registers might have different fields within them, often on a sub-byte width. This makes accessing different bits or lengths of bits somewhat obtuse I think. There are a couple of methods you could use to approach this issue normally, all of which I think are inadequate, so I created my own. It features standard defined behaviour, memory safety, named access to every bit field, access permissions, and more. In this document I will first talk about the conventional methods. Then I will go over their shortcommings, and finally I will introduce my method, how it works, and how to use it. At the end I will briefly discuss small weaknesses I see in my solution along with future improvements I intend on pursuing.

I decided to create this application in Rust as an exercise to combine solving an issue I am having with my own projects using registers, with learning Rust as I am still pretty new to the language.

## Use Case
Let's say we are working with a PCIe driver, and we need to control PCIe registers. For this example we will work with the Link Capabilities Register, the definition of which can be found in the **PCI Express Base r3.0** specification in **Section 7.8.6**. The register definition is as follows:

| Bit Location | Description |
| --- | --- |
| 3:0 | Max Link Speed |
| 9:4 | Max Link Width |
| 11:10 | Active State Power Management Support |
| 14:12 | L0s Exit Latency |
| 17:15 | L1 Exit Latency |
| 18 | Clock Power Management |
| 19 | Surprise Down Error Reporting Capable |
| 20 | Data Link Layer Link Active Reporting Capable |
| 21 | Link Bandwidth Notification Capability |
| 22 | ASPM Optionality Compliance |
| 23 | Reserved |
| 31:24 | Port Number |

Let's say we want to figure out if ASPM is supported, we will need to check bits 11:10.

The encoding for those bits is:

| Encoding | Meaning |
| --- | --- |
| 00b | No ASPM Support |
| 01b | L0s Supported |
| 10b | L1 Supported |
| 11b | L0s and L1 Supported |

Let's imagine that you already have some method called `uint32_t read_register(uint32_t offset)` which reads a register in from DRAM or MMIO or something, along with another method `void write_register(uint32_t offset, uint32_t value)` for writing a value to a register.

Let's also imagine that the link capabilites register is currently `0xdeadbeef`.

The bit representation of `0xdeadbeef` can be seen below. This shows that the bits 11:10 are `3` or `0b11`.

![image](https://github.com/regerj/jacobs_register_helper/assets/71850611/b11440f5-59f6-409e-8c05-6b65fc58620c)

## Method 1: Bit Shifting and Masking
Our first, and most often used method, is with bit shifting and masking to retrieve only a certain chunk of bits from within a register. Our C++ code for such a method might look something like this:

```cpp
// Read the register from memory
uint32_t link_capabilites_reg = read_register(0x0C);
uint32_t aspm_support = 0x0;

aspm_support = (link_capabilites_reg >> 10) | 0b11;
```

This bit shifts the register by 10 bits to the right, so that the tenth bit is now the zero'th bit. Next, it is bit masked with `0b11` to only read the two bits we are interested. This will result in `aspm_support == 0b11`.

This method works well from a functionality standpoint, and perhaps a comment above could indicate where these numbers are coming from, but it is not reproducable without going to the documentation. Checking certain fields of a register may be done repeatedly throughout the codebase, and each time you will have to reference the documentation to retrieve these magic numbers (the 10th bit start and the 2 bit width). This problem could possibly be mitigated by using some constant definitions, such as a `#define` for the field start and another for the field width. This would change our solution to be more like this:

```cpp
// Some constant definitions header
#define ASPM_SUPPORT_START 10
#define ASPM_SUPPORT_WIDTH 0b11

// main.cpp
// Read the register from memory
uint32_t link_capabilites_reg = read_register(0x0C);
uint32_t aspm_support = 0x0;

aspm_support = (link_capabilites_reg >> ASPM_SUPPORT_START) | ASPM_SUPPORT_WIDTH;
```

Adding this to the approach mitigates the need for looking up the magic numbers each time, but it introduces it's own set of problems.

### Pros
1. Standard defined behaviour.

### Cons
1. Potential name conflicts. For example, some bit fields have extremely common names like `link_width` which would collide. You could mitigate this with more verbose names, but that can't be the best solution...
2. Misuse of these constants for different registers. The constant definition file which contains all of these magic numbers will likely contain the constants for a number of different registers, possibly dozens or hundreds. There is nothing preventing you from unintentionally using a constant meant for, say, the Device Status register on a Link Capabilites register, which could cause disasterous effects.
3. No code insights into the contents of the register. This approach still requires you to reference the documentation every time you go to access a member of a register, in order to know what fields are present, and infer from that the names of the constants you would need to use. This could potentially be mitigated by organizing your constant defines such that they are grouped by the register which owns them, which could be described in a comment above the constants. This would mean you no longer have to reference the documentation, but you would have to find the register in the constant header file. There has got to be a more robust solution...

You could provide MACROs to simplify the access to certain fields, perhaps it might look something like this:

```cpp
// pcie_registers.h

#define ASPM_SUPPORT_START 10
#define ASPM_SUPPORT_WIDTH 0b11

#define GET_ASPM_SUPPORT(link_capabilites_register) (link_capabilites_register >> ASPM_SUPPORT_START) | ASPM_SUPPORT_WIDTH

// main.cpp
// Read the register from memory
uint32_t link_capabilites_reg = read_register(0x0C);
uint32_t aspm_support = 0x0;

aspm_support = GET_ASPM_SUPPORT(link_capabilites_reg);
```

This makes the code look a bit cleaner, but it fails to mitigate the major issues discussed above, it simply moves them or changes in what way they come up.

## Method 2: Bit Fields
Another possible method that fixes many of these problems is using bitfields within a struct. This allows you to assign names to bit length defined fields. Your registers could then be defined as follows:

```cpp
// pcie_registers.h

struct link_capabilities_register {
  uint32_t max_link_speed : 4;
  uint32_t max_link_width : 6;
  uint32_t aspm_support : 2;
  uint32_t l0s_exit_latency : 3;
  uint32_t l1_exit_latency : 3;
  uint32_t clock_power_management : 1;
  uint32_t surprise_down_error_reporting_capable : 1;
  uint32_t data_link_layer_link_active_reporting_capable : 1;
  uint32_t link_bandwidth_notification_capability : 1;
  uint32_t aspm_optional_compliance : 1;
  uint32_t reserved : 1;
  uint32_t port_number : 8;
};
```

This, at face value, seems to be a **_great_** solution. We now have named bit fields within a struct, which allows us to instantiate an object of a static type `link_capabilites_reg` and we have insight into the fields of this register through LSP autocompletion. This solution comes with a different set of problems though. Most immediately is there is no way to nicely retrieve the actual full register value again, which you would need for writing the register back. You would need to turn your object back into a `uint32_t` for the write call. This is not possible with this method without some sketchy and probably very undefined pointer casting. There is no way to actually access the underlying complete register in order to populate it from a `read_register()` call which would likely return a `uint32_t` or send it to a `write_register()` call which would expect a `uint32_t`.

We could alleviate this problem by wrapping this struct in a union. That implementation might look something like this:

```cpp
// pcie_registers.h

union link_capabilites_register {
  uint32_t raw;
  struct link_capabilities_register {
    uint32_t max_link_speed : 4;
    uint32_t max_link_width : 6;
    uint32_t aspm_support : 2;
    uint32_t l0s_exit_latency : 3;
    uint32_t l1_exit_latency : 3;
    uint32_t clock_power_management : 1;
    uint32_t surprise_down_error_reporting_capable : 1;
    uint32_t data_link_layer_link_active_reporting_capable : 1;
    uint32_t link_bandwidth_notification_capability : 1;
    uint32_t aspm_optional_compliance : 1;
    uint32_t reserved : 1;
    uint32_t port_number : 8;
  } data;
};

// main.cpp
// Read the register from memory
link_capabilites_register link_capabilites_reg{};
link_capabilites_reg.raw = read_register(0x0C);
uint32_t aspm_support = 0x0;

aspm_support = link_capabilites_reg.data.aspm_support;

```

You would then be able to access the raw register value using `link_capabilites_reg.raw`, but this still is riddled with issues.

### Pros
1. Named access to fields.
2. LSP insight into register contents (less going to the documentation).

### Cons
1. Reserved bits must be declared in the bitfield, otherwise locations may become messed up. This means that it is possible for someone to write to those reserved bits unintentionally, without violating the access pattern.
2. This approach is FULL of implementation defined behaviour. The C++ standard makes few gurantees about the functioning of bitfields within a struct, which leaves it up to the implementation. This is because the C++ standard views bitfields as a way of compacting data, decidedly not as a method of defining registers. This creates unportable and fragile code. This method is advised against very strongly.

## My Solution
My solution involves some MACRO based metaprogramming. Essentially, it provides a set of MACROs for creating register definitions of varying widths. An implementation of the above problem using my header might look something like this:

```cpp
// pcie_registers.h
#include <jacobs_register_helper.h> // My implementation file

DECLARE_REGISTER_32(
  link_capabilites_register,
  max_link_speed, 0, 3,
  max_link_width, 4, 9,
  aspm_support, 10, 11,
  l0s_exit_latency, 12, 14,
  l1_exit_latency, 15, 17,
  clock_power_management, 18, 18,
  surprise_down_error_reporting_capable, 19, 19,
  data_link_layer_link_active_reporting_capable, 20, 20,
  link_bandwidth_notification_capability, 21, 21,
  aspm_optionality_compliance, 22, 22,
  port_number, 24, 31
);

// main.cpp
#include "pcie_registers.h"

int main() {
  link_capabilites_register reg;
  reg.set_register_value(read_register(0x0C));
  uint32_t aspm_support = 0x0;
  aspm_support = reg.get_aspm_support();
}
```

The setup here is that you would have some header file containing repeated calls to my provided MACROs to define the structure of all the registers you might read/write in your code. The arg list looks like `(NAME, FIELD, START, END, FIELD, START, END, ...)`. You provide a name for your register type as the first argument, and you follow it by a series of `(FIELD, START, END)` where `FIELD` represents the name of the field, `START` represents the beginnig bit, and `END` represents the ending bit (inclusive). This allows for readable, easy to create register definitions that clearly state the start and end bits of the field for self documenting code.

This would allow these register definitions to be used anywhere in the code so long as that file `#include`s your register definitions file. So lets see how it works, and how it fixes the problems of the previous two approaches.

The `DEFINE_REGISTER_XX` macro first instantiates a class of the same name as is provided to the macro. Within this class, it recursively iterates through each set of `(FIELD, START, END)`, and creates two methods for each. It creates a getter and a setter. They follow the naming conventions of `get_ + FIELD()` and `set_ + FIELD()`. These methods use bit operations to retreive the requested field from an internal private `uint32_t` which represents the register's actual value. The declaration of these methods look like the following (assuming 32 bit MACRO):

```cpp
uint32_t get_some_field_name() const;
bool set_some_field_name(uint32_t value);
```

Here, the returned `bool` from the `set` method represents whether that set was successful or not. The situation where a failure might occur is when the caller attempts to assign a value that is too large for that bit field. For example, we could not assign `12` to a bitfield that is 3 bits wide, that would be an overflow. I considered preventing this using asserts instead of a success/fail return value, but I figured I would avoid any kind of runtime exceptions for portability and ease of use.

The class also contains three more methods for interacting with the register as a whole. These are:

```cpp
uint32_t get_register_value() const;
void clear_register_value();
void set_register_value(uint32_t value);
```

These methods are the same accross all register definitions, as they are for interacting with the entire register. This may be important when you want to reuse a register object for another read, you might want to clear it. When reading it in from memory, you will want to set it. Finally, when passing it to a theoretical `write_register()` method, you might want to get the raw value.

So how does this fix our problems?

It combines the strengths of both of the previously mentioned methods, while eliminating their pitfalls. It leverages the implementation agnostic behaviour of the first method of bitshifting and masking for the accesses, while maintaining the strictly defined and descriptively named accessible fields from the second method using bitfields. This is LSP autocomplete compatible because the methods are defined in the class which has been instantiated. This also has an added benefit of allowing reserved bits to be entirely inaccessible because no get or set method is generated for them if you just omit them from your arg list in the MACRO call.

Like the second method, it also prevents naming conflict for fields with common names like `port_width` because it is all within a class scope.

Static asserts are present to prevent instantiation of a register with accesses beyond the bounds of the underlying integer.

## Weaknesses

- A potential weakness of this approach is that (currently) I cannot figure out how to implement anything preventing fields from overlapping. This is really a user error, but the entire idea of this project is to prevent as much user error as possible. Maybe I will just call it a feature for supporting registers with dynamic definitions :sunglasses:.

- Another weakness is that it is currently only C++ compatible. This is because it uses classes, but more importantly it is because the `__VA_OPT__` that the recursive MACRO call relies on is >= C++20.

- There may be some level of memory overhead (not much runtime overhead I don't think...) in the object instantiations, but I think that is a small price to pay for a considerably more robust implementation of register support.

## Planned Future Work

- Add 8 bit and 64 bit register width support
- Port to Rust (or at least try) 🦞

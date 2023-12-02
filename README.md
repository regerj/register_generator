# Register Generator

[Documentation](docs/DOCS.md) on how to use this solution is available in DOCS.md. This README will cover the  current solutions, their problems, the need for a better solution, and cover how my solution fixes most of these weaknesses.

## Contents
<!--ts-->
  * [Description](#description)
  * [Use Case](#use-case)
  * [Method 1 Bit Shifting and Masking](#method-1-bit-shifting-and-masking)
  * [Method 2 Bit Fields](#method-2-bit-fields)
  * [My Solution](#my-solution)
  * [Weaknesses](#weaknesses)
  * [Planned Features](#planned-features)
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
My solution uses a JSON file specifying registers within a family and their fields. This JSON will be used by the application to generate classes for each register defined in the JSON. Each register will inherit from a base class of the same register width. For example, each register of 16 bit width will inherit from the following base class:

```cpp
// This file was automatically generated by a register generation tool
// https://github.com/regerj/register_generator
// Any changes to this file may be overwritten on subsequent generations

#pragma once

#include <cstdint>

class Register16 {
public:
	Register16() = default;
	inline uint16_t get_register_value() const { return register_raw; };
	inline void clear_register_value() { register_raw = 0x0; };
	inline void set_register_value(uint16_t value) { register_raw = value; };
protected:
	uint16_t register_raw = 0x0;
};
```
This defines a more abstract concept of a register, in which only getting, clearing, and setting the entire register is available. This inheritance is justified, because it allows registers of **ANY** type to be passed into an API designed around this inheritance. For example, when working on low level systems, we often have some function like:

```cpp
void write_pcie_hif_register(uint32_t address, uint32_t value);
```

which will write a PCIe HIF Register at a certain address. This kind of call is register agnostic, meaning no concept of a "register" is enforced in the API. It takes a generic `uint32_t`. We can fix that. Secondly, the specific register is also generic, meaning it does not write one specific register, it writes **some** register. We want to preserve that. With this inheritance, we would now have a method that looks more like

```cpp
void write_pcie_hif_register(uint32_t address, const Register16 &value);
```

which does both. It simultaneously enforces the idea of a "register" onto the argument, while remaining agnostic to any particular register, only its width.

Now lets look at an actual register definiton. These classes will be generated by the tool in a file called `[SOME_NAME]Registers.h`. This name is configurable in the JSON, which I will go over later in the document.

```cpp
class TestRegister1 : public Register16 {
public:
	TestRegister1() : Register16() {};

	// Get methods
	inline uint16_t get_lo() const {
		uint16_t buffer = register_raw >> 0;
		return buffer & (UINT16_MAX >> (16 - 1 - (7 - 0)));
	}
	inline uint16_t get_hi() const {
		uint16_t buffer = register_raw >> 8;
		return buffer & (UINT16_MAX >> (16 - 1 - (15 - 8)));
	}

	// Set methods
	inline bool set_lo(uint16_t value) {
		if (value >= (1 << (7 - (0 - 1)))) {
			return false;
		}
		uint16_t mask = static_cast<uint16_t>(~((UINT16_MAX >> (16 - 1 - (7 - 0))) << 0));
		register_raw &= mask;
		value = value << 0;
		register_raw |= value;
		return true;
	}
	inline bool set_hi(uint16_t value) {
		if (value >= (1 << (15 - (8 - 1)))) {
			return false;
		}
		uint16_t mask = static_cast<uint16_t>(~((UINT16_MAX >> (16 - 1 - (15 - 8))) << 8));
		register_raw &= mask;
		value = value << 8;
		register_raw |= value;
		return true;
	}
};
```

As you can see, the test register inherits from our `Register16` class which the tool will also generate for you in a seperate header called `Register16.h`. This is in a seperate header because we may want to only include the higher level concept of a register in a higher level of the hierarchy, and the specific register instantiations need only be scoped to a lower level of the architecture. This is a very simple register definition, with only some generic "hi" and "lo" bytes within it, but these fields are all configurable through the JSON file, with support for as many fields as you could reasonably want.

These methods make use of bit operations to fix the shortcomings of the previously mentioned struct type-punning method in that they are endianness agnostic and standard defined. It also solves the shortcomings of the standard method of bit operations with constants because now permissions are enforced (access methods are generated only based on their permission defined in the JSON) and now there is no confusion as to which constants and masks can be used on a particular register, it is all defined as named methods. This improves register safety, and speeds up development, especially with the use of an LSP.

Now lets look at an example JSON to see how we can configure our register generations and end up with nicely defined classes like shown above.

```json
{
    "register_family":"HIF",
    "register_family_widths":[16,32],
    "registers":[
        {
            "name":"TestRegister1",
            "size":16,
            "fields":[
                {
                    "name":"lo",
                    "lsb":0,
                    "msb":7,
                    "read":true,
                    "write":true
                },
                {
                    "name":"hi",
                    "lsb":8,
                    "msb":15,
                    "read":true,
                    "write":true
                }
            ]
        },
        {
            "name":"TestRegister2",
            "size":32,
            "fields":[
                {
                    "name":"LOW",
                    "lsb":0,
                    "msb":15,
                    "read":true,
                    "write":true
                },
                {
                    "name":"HIGH",
                    "lsb":16,
                    "msb":31,
                    "read":true,
                    "write":true
                }
            ]
        }
    ]
}
```

Every file must contain three key-pair mappings. It will need a `register_family` containing a string name for the family of registers that will be defined in this JSON file, `register_family_widths` containing an array of all register widths that will be present in this family of registers, and finally an array of actual registers under the key `registers`.

Each of those registers must contain three keys itself. The first is the `name` of the register, this will be the name of the instantiated class for that register. Next it will need the register `size`. This will be an integer either 8, 16, 32, or 64. I do not currently support any other lengths of registers, and do not have any current plans on adding it. Finally, it will have an array named `fields` which will contain JSON objects for each field in the register.

Each filed needs to contain five keys. First, once again, is `name` which will be the name of the field, and will be used to name the access methods to that field within the class. Second and third will be `lsb` and `msb` which represent the **INCLUSIVE** bit bounds of the register. Lastly will be `read` and `write` which are boolean values representing if read and write access is allowed to that particular field. This controls if a get or set method is generated for that field.

## Weaknesses

- There is no JSON schema enforcement at the moment, as I am still very new to Rust. I will be adding this in the future, but I need to get some free time and read the documentation. This is a top priority on this project.

- Adding registers to the JSON can be somewhat tedious. Vim keybinds make this a little less tedious, but it is still not optimal. I plan to provide another binary which will provide a CLI for generating a new JSON file or appending new registers to it.

- There may be some level of memory overhead (not much runtime overhead I don't think...) in the object instantiations, but I think that is a small price to pay for a considerably more robust implementation of register support.

- There is nothing preventing overlapping register fields at the moment. I am debating whether or not to enforce this, as I imagine some niche registers may be somewhat polymorphic, and at the end of the day overlapping bit boundaries is a user error. However, the entire point of this project was to prevent user errors to a reasonable degree, so I may or may not implement this.

## Planned Features

- Add JSON schema enforcement.

- Add CLI for generating and appending register definitions to JSON.

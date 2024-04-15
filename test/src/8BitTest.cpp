#include <gtest/gtest.h>

#include <8BitRegisters.h>

TEST(Test8Bit, GetTest) {
    HighLow reg;

    // Test 0 init
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0x0);

    // Test with max
    reg.set_register_value(0x0F);

    EXPECT_EQ(reg.get_low(), 0xF);
    EXPECT_EQ(reg.get_high(), 0x0);

    reg.set_register_value(0xF0);

    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0xF);

    // Test with half
    reg.set_register_value(0x07);

    EXPECT_EQ(reg.get_low(), 0x7);
    EXPECT_EQ(reg.get_high(), 0x0);

    reg.set_register_value(0x70);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0x7);
}

TEST(Test8Bit, SetTest) {
    HighLow reg;

    EXPECT_EQ(reg.get_register_value(), 0x0);

    EXPECT_EQ(reg.set_low(0x0), true);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(0x7), true);
    EXPECT_EQ(reg.get_low(), 0x7);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x07);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(0xF), true);
    EXPECT_EQ(reg.get_low(), 0xF);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0F);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(0x10), false);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(0xFF), false);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();
    
    EXPECT_EQ(reg.set_high(0x0), true);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(0x7), true);
    EXPECT_EQ(reg.get_high(), 0x7);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x70);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(0xF), true);
    EXPECT_EQ(reg.get_high(), 0xF);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0xF0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(0x10), false);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(0xFF), false);
    EXPECT_EQ(reg.get_high(), 0x0);
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();
}


TEST(Test8Bit, NegGetTest) {
    NegHighLow reg;

    // Test 0 init
    EXPECT_EQ(reg.get_low(), 0x0);
    EXPECT_EQ(reg.get_high(), 0x0);

    // Test with max
    reg.set_register_value(0b0000'1000);

    EXPECT_EQ(reg.get_low(), -8);
    EXPECT_EQ(reg.get_high(), 0);

    reg.set_register_value(0b1000'0000);

    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_high(), -8);

    // Test with half
    reg.set_register_value(0x07);

    EXPECT_EQ(reg.get_low(), 7);
    EXPECT_EQ(reg.get_high(), 0);

    reg.set_register_value(0x70);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_high(), 7);
}

TEST(Test8Bit, NegSetTest) {
    NegHighLow reg;

    EXPECT_EQ(reg.get_register_value(), 0x0);

    EXPECT_EQ(reg.set_low(0), true);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(7), true);
    EXPECT_EQ(reg.get_low(), 7);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x07);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(-8), true);
    EXPECT_EQ(reg.get_low(), -8);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_register_value(), 0b0000'1000);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(-9), false);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(8), false);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();
    
    EXPECT_EQ(reg.set_high(0), true);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(7), true);
    EXPECT_EQ(reg.get_high(), 7);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x70);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(-8), true);
    EXPECT_EQ(reg.get_high(), -8);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_register_value(), 0b1000'0000);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(-9), false);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(8), false);
    EXPECT_EQ(reg.get_high(), 0);
    EXPECT_EQ(reg.get_low(), 0);
    EXPECT_EQ(reg.get_register_value(), 0x0);
    reg.clear_register_value();
}

#include <cstdlib>
#include <gtest/gtest.h>

#include <32BitRegisters.h>

#define ZERO 0x00
#define MAX 0xFFFF'FFFF
#define HALF_UNSIG 0x7FFF
#define MAX_UNSIG 0xFFFF
#define MIN_SIG -32'768
#define MAX_SIG 32'767

#define HALF_SHIFT 16

TEST(Test32Bit, GetTest) {
    HighLow_32 reg;

    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(MAX_UNSIG);
    EXPECT_EQ(reg.get_register_value(), MAX_UNSIG);

    EXPECT_EQ(reg.get_low(), MAX_UNSIG);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(MAX_UNSIG << HALF_SHIFT);

    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), MAX_UNSIG);

    reg.set_register_value(HALF_UNSIG);

    EXPECT_EQ(reg.get_low(), HALF_UNSIG);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(HALF_UNSIG << HALF_SHIFT);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), HALF_UNSIG);
}

TEST(Test32Bit, SetTest) {
    HighLow_32 reg;

    EXPECT_EQ(reg.get_register_value(), ZERO);

    EXPECT_EQ(reg.set_low(ZERO), true);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(HALF_UNSIG), true);
    EXPECT_EQ(reg.get_low(), HALF_UNSIG);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), HALF_UNSIG);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MAX_UNSIG), true);
    EXPECT_EQ(reg.get_low(), MAX_UNSIG);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), MAX_UNSIG);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MAX_UNSIG + 1), false);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MAX), false);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();
    
    EXPECT_EQ(reg.set_high(ZERO), true);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(HALF_UNSIG), true);
    EXPECT_EQ(reg.get_high(), HALF_UNSIG);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), HALF_UNSIG << HALF_SHIFT);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MAX_UNSIG), true);
    EXPECT_EQ(reg.get_high(), MAX_UNSIG);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), MAX_UNSIG << HALF_SHIFT);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MAX_UNSIG + 1), false);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MAX), false);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();
}


TEST(Test32Bit, NegGetTest) {
    NegHighLow_32 reg;

    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(MIN_SIG & MAX_UNSIG);

    EXPECT_EQ(reg.get_low(), MIN_SIG);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(MIN_SIG << HALF_SHIFT);

    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), MIN_SIG);

    reg.set_register_value(MAX_SIG);

    EXPECT_EQ(reg.get_low(), MAX_SIG);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(MAX_SIG << HALF_SHIFT);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), MAX_SIG);
}

TEST(Test32Bit, NegSetTest) {
    NegHighLow_32 reg;

    EXPECT_EQ(reg.get_register_value(), ZERO);

    EXPECT_EQ(reg.set_low(ZERO), true);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MAX_SIG), true);
    EXPECT_EQ(reg.get_low(), MAX_SIG);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), MAX_SIG);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MIN_SIG), true);
    EXPECT_EQ(reg.get_low(), MIN_SIG);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), MIN_SIG & MAX_UNSIG);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MIN_SIG - 1), false);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_low(MAX_SIG + 1), false);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();
    
    EXPECT_EQ(reg.set_high(ZERO), true);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MAX_SIG), true);
    EXPECT_EQ(reg.get_high(), MAX_SIG);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), MAX_SIG << HALF_SHIFT);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MIN_SIG), true);
    EXPECT_EQ(reg.get_high(), MIN_SIG);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), abs(MIN_SIG << HALF_SHIFT));
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MIN_SIG - 1), false);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MAX_SIG + 1), false);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();
}

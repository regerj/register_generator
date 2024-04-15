#include <cstdlib>
#include <cstdint>
#include <gtest/gtest.h>

#include <64BitRegisters.h>

#define ZERO 0x00
#define MAX 0xFFFF'FFFF'FFFF'FFFF
#define HALF_UNSIG 0x7FFF'FFFF
#define MAX_UNSIG 0xFFFF'FFFF
#define MIN_SIG -2'147'483'648
#define MAX_SIG 2'147'483'647

#define HALF_SHIFT 32

TEST(Test64Bit, GetTest) {
    HighLow_64 reg;

    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value(MAX_UNSIG);
    EXPECT_EQ(reg.get_register_value(), MAX_UNSIG);

    EXPECT_EQ(reg.get_low(), MAX_UNSIG);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value((uint64_t)MAX_UNSIG << HALF_SHIFT);

    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), MAX_UNSIG);

    reg.set_register_value(HALF_UNSIG);

    EXPECT_EQ(reg.get_low(), HALF_UNSIG);
    EXPECT_EQ(reg.get_high(), ZERO);

    reg.set_register_value((uint64_t)HALF_UNSIG << HALF_SHIFT);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), HALF_UNSIG);
}

TEST(Test64Bit, SetTest) {
    HighLow_64 reg;

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

    EXPECT_EQ(reg.set_low((uint64_t)MAX_UNSIG + 1), false);
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
    EXPECT_EQ(reg.get_register_value(), (uint64_t)HALF_UNSIG << HALF_SHIFT);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high(MAX_UNSIG), true);
    EXPECT_EQ(reg.get_high(), MAX_UNSIG);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), (uint64_t)MAX_UNSIG << HALF_SHIFT);
    reg.clear_register_value();

    EXPECT_EQ(reg.set_high((uint64_t)MAX_UNSIG + 1), false);
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


TEST(Test64Bit, NegGetTest) {
    NegHighLow_64 reg;

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

    reg.set_register_value((int64_t)MAX_SIG << HALF_SHIFT);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_high(), MAX_SIG);
}

TEST(Test64Bit, NegSetTest) {
    NegHighLow_64 reg;

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

    EXPECT_EQ(reg.set_low((int64_t)MAX_SIG + 1), false);
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
    EXPECT_EQ(reg.get_register_value(), (int64_t)MAX_SIG << HALF_SHIFT);
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

    EXPECT_EQ(reg.set_high((int64_t)MAX_SIG + 1), false);
    EXPECT_EQ(reg.get_high(), ZERO);
    EXPECT_EQ(reg.get_low(), ZERO);
    EXPECT_EQ(reg.get_register_value(), ZERO);
    reg.clear_register_value();
}

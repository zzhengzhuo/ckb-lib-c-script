#include <gtest.h>
#include "ckb_smt.h"

TEST(CKB_STM, HandlesZeroInput)
{
    uint8_t root[32];
    uint32_t smt_pair_len = 1;
    uint8_t keys[32];
    uint8_t values[32];
    uint8_t proof[32];
    uint32_t proof_length = 0;

    EXPECT_EQ(ckb_smt_verify(root,smt_pair_len,keys,values,proof,proof_length), 0);
}
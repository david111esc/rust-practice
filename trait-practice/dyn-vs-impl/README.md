# 測試 Trait 關鍵字的影響

## GOAL
比較當 trait Animal 作為 borrow 參數輸入函數中時，使用 &impl Animal 和 &dyn Animal 的差異。
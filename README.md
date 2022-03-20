# balance-parentheses

## Minimum number of parenthesis to be added to make it valid

Difficulty Level: Medium

Given a string S of parentheses ‘(‘ or ‘)’ where, `0 ≤ len(S) ≤ 105`. The task is to find a minimum number of parentheses ‘(‘ or ‘)’ (at any positions) we must add to make the resulting parentheses string is valid.

**Examples:**
```
Input: str = "())"
Output: 1
One '(' is required at beginning.

Input: str = "((("
Output: 3
Three ')' is required at end.
```

**Approach**: we keep the track of balance of the string i:e the number '(' minus the number of ')'.
A string is valid if it's balance is 0. If it's ever negative, we must add a '(' bracket at the beginning. Also, if the balance of string is positive, we must add a ')' brackets at the end.

**Time Complexity:** O(N), where N is the length of S. 
**Space Complexity:** O(1).
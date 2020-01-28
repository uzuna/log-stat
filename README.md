# log-stat

journallogの簡単な統計処理を行って適切なロギングができているのか可視化をする


## alloc result

```
# String版
==18605== HEAP SUMMARY:
==18605==     in use at exit: 384 bytes in 6 blocks
==18605==   total heap usage: 138 allocs, 132 frees, 39,804 bytes allocated

# &str版
==19460== HEAP SUMMARY:
==19460==     in use at exit: 384 bytes in 6 blocks
==19460==   total heap usage: 139 allocs, 133 frees, 40,734 bytes allocated
```
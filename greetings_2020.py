
greetings = ["world hello", "year happy new", "吉 鼠 大 年"]

for g in greetings:
    words = sorted(g.split())
    if "\u4e00" <= words[0] <= "\u9fff":
        words.sort(reverse=True)
    print(" ".join(words).title()) 
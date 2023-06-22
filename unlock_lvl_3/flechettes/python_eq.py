NB_LETTRES = int(input())
ALPHABET = "abcdefghijklmnopqrstuvwxyz"

lines = []
for y in range(NB_LETTRES):
    first_part = ""
    for x in range(NB_LETTRES - 1):
        x = min(x, y)
        first_part += ALPHABET[x]
    last_part = first_part[::-1]

    line = "{}{}{}".format(first_part, ALPHABET[y], last_part)
    lines.append(line)

bottom_part = list(reversed(lines[:NB_LETTRES-1]))
lines.extend(bottom_part)

rendered_target = "\n".join(lines)
print(rendered_target)
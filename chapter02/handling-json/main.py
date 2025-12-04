import json
import sys

s = 0
for line in sys.stdin:
    value = json.loads(line)
    s += value['value']
    s += len(value['name'])

print(s)
''' echo '{ "name": "Rachelle Ferguson", "value": 948129 }' | python main.py '''

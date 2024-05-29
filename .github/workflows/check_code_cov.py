import sys, json

dt=json.load(sys.stdin)['data'][0]['totals']

assert(dt['branches']['count'] == dt['branches']['covered'])
assert(dt['functions']['count'] == dt['functions']['covered'])
assert(dt['lines']['count'] == dt['lines']['covered'])
assert(dt['mcdc']['count'] == dt['mcdc']['covered'])
assert(dt['regions']['count'] == dt['regions']['covered'])

#!/usr/bin/python

import urllib.parse
import urllib.request

# A1 = 'https://atstest.xyz/addcom'
# A2 = 'Herman Munster'
# A3 = 'herman@monster.com'
# A4 = 'This is the first comment lets see if it works'
# A5 = '5'

# url_raw = A1 + '/' + A2 + '/' + A3 + '/' + A4 + '/' + A5

# # replace whitespace with %20
# url = urllib.parse.quote(url_raw, safe='/:')
# # make an http request to the url
# urllib.request.urlopen(url)

url = 'https://atstest.xyz/test'
req = urllib.request.Request(url)
response = urllib.request.urlopen(req)
print(response.read().decode('utf-8'))
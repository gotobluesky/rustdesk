#!/usr/bin/env python3

import os
import glob
from tabnanny import check

def main():
   print('export const LANGS = {')
   for fn in glob.glob('../hbb/src/lang/*'):
      lang = os.path.basename(fn)[:-3]
      print('  %s: {'%lang)
      for ln in open(fn):
         ln = ln.strip()
         if ln.startswith('("'):
            toks = ln.split('", "')
            assert(len(toks) == 2)
            a = toks[0][2:]
            b = toks[1][:-3]
            print('    "%s": "%s",'%(a, b))
      print('  },')
   print('}')
   check_if_retry = ['', False]
   KEY_MAP = ['', False]
   for ln in open('../hbb/src/client.rs'):
      ln = ln.strip()
      if 'check_if_retry' in ln:
         check_if_retry[1] = True
         continue
      if ln.startswith('}') and check_if_retry[1]:
         check_if_retry[1] = False
         continue
      if check_if_retry[1]:
         check_if_retry[0] += ln + '\n'
      if 'KEY_MAP' in ln:
         KEY_MAP[1] = True
         continue
      if '.collect' in ln and KEY_MAP[1]:
         KEY_MAP[1] = False
         continue
      if KEY_MAP[1] and ln.startswith('('):
         toks = ln.split('", Key::')
         assert(len(toks) == 2)
         a = toks[0][2:]
         b = toks[1].replace('ControlKey(ControlKey::', '').replace("Chr('", '').replace("' as _)),", '').replace(')),', '')
         KEY_MAP[0] += '  "%s": "%s",\n'%(a, b)
   print()
   print('export function checkIfRetry(msgtype: string, title: string, text: string) {')
   print('  return %s'%check_if_retry[0].replace('to_lowercase', 'toLowerCase').replace('contains', 'indexOf').replace('!', '').replace('")', '") < 0'))
   print(';}')
   print()
   print('export const KEY_MAP: any = {')
   print(KEY_MAP[0])
   print('}')
      


main()
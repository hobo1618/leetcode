Todo

1. Create list of exercises and save them to nushell as autocomplete??
  1. create list of exercises
2. Convert curl command plus pipeline into a nushell command with params
3. Factor out tokens into an ENV variable so I can push to GH
4. Figure out where the empty fn and signature are (which curl request)

Pipeline

1. curl command with name of exercise to json
2. grab relevant field from json and convert to html
3. clean html by replacing:
  1. \n with <br/> tags
  2. \t with a space plus a hyphen ' -'
  3. <sup> with ^ and </sup> with nothing
  4. <sub> with _ and </sub> with nothing
  5. &apos; with ' (etc)
4. convert html to markdown
5. write file to /examples/[exercise].md

API 

leetcode add `merge-strings-alternately`




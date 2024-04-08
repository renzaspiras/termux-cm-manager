import os

binary = os.listdir("./bin")

for j in binary:
  executable = j.split(".")[0]
  extention = j.split(".")[1]

  if(extention == "rs"):
    os.system(f"rustc ./neo/bin/{j} -o compiled/{executable}")  
  elif(extention == "cpp"):
    os.system(f"g++ ./neo/bin/{j} -o compiled/{executable}")
#! /usr/bin/python3
import time
def main():
    counter = 0
    while counter<10:
        counter+=1
        print(f"{counter}",flush=True)
        time.sleep(0.1)

if __name__=="__main__":
    main()
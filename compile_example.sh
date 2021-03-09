#!/bin/zsh

docker exec ubuntu sh -c "nasm -felf64 /mac/Projects/KeenTeam/cwe_checker/cwe_checker_for_learn/playground/$1.asm && ld /mac/Projects/KeenTeam/cwe_checker/cwe_checker_for_learn/playground/$1.o -o /mac/Projects/KeenTeam/cwe_checker/cwe_checker_for_learn/playground/$1 && rm /mac/Projects/KeenTeam/cwe_checker/cwe_checker_for_learn/playground/$1.o"
analyzeHeadless ./playground/ playground -overwrite -import playground/$1 -postScript PcodeExtractor.java playground/$1_pcode.json
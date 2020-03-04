# TextExcelPort [![Build Status](https://travis-ci.org/Triscuit-circuit/TextExcelPort.svg?branch=master)](https://travis-ci.org/Triscuit-circuit/TextExcelPort)

A mini version of a Microsoft Excel like program (inputs done by command line) written in Rust. 



Following features are being worked on for this program:
- implementation of a shunting yard algorithm to do PEMDAS calculations for formulas 
- implemtation of formulas that read other parts of the grid
- save and export as CSV (Comma Seperated Values) along with loading from comma seperated values 
- error detection to avoid panics
- sorting values on the grid
- multiple grid files

Features that will be implemented after other features:
- implemtation of a graphical user interface using the Azul crate or Conrod
- options menu

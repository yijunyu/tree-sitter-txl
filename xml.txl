% Rust parser and pretty printer
% Copyright 2020, Huawei Technologies Co. Ltd.

include "tree-sitter-xml-hover.grm"
% include "rust3.grm"

% Uncomment this line if you'd like pure un-pretty-printed XML output
% #pragma -indent 0

function main
    replace [program]
        XMLdoc [program]
    by
        XMLdoc [checkWellFormed]
end function

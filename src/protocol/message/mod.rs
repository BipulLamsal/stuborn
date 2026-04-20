/*
   +---------------------+
   |        Header       |
   +---------------------+
   |       Question      | the question for the name server
   +---------------------+
   |        Answer       | RRs answering the question
   +---------------------+
   |      Authority      | RRs pointing toward an authority
   +---------------------+
   |      Additional     | RRs holding additional information
   +---------------------+

*/
mod header;
mod question;

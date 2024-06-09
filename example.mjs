import { sum, fibonacci, runCallback, QueryEngine } from "./index.js";

console.log("From native", sum(40, 2));
console.log("From native", fibonacci(20));
runCallback((s) => console.log(`${s} Hello from Rust!`));
const qe = new QueryEngine("my_query");
console.log(qe.queryName);
qe.queryName = "new_query_name";
console.log(qe.queryName);

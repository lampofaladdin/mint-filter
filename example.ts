import { Mint } from "./index";

class Demo {
  keywords: string[];
  constructor(keywords: string[]) {
    this.keywords = keywords;
  }

  getData() {
    return this.keywords;
  }
}

const demo = new Mint(new Array(1000).fill("dog"));
console.time("1");
 demo.getData();
 demo.getData();
 demo.getData();
 demo.getData();
 demo.getData();
console.timeEnd("1");

const demo2 = new Demo(new Array(1000).fill("dog"));
console.time("2");
demo2.getData();
demo2.getData();
demo2.getData();
demo2.getData();
demo2.getData();
console.timeEnd("2");

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
const res =  demo.filter('hi');
console.error('node res:',res);



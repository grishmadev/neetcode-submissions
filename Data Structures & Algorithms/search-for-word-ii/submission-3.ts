const A: number = 'a'.charCodeAt(0);
class Trie {
  children: (Trie | null)[] = [];
  word: string = "";
  constructor(w?: string) {
    if (w) this.word = w;
    for (let i = 0; i < 26; i++) {
      this.children.push(null);
    }
  }
}

class Solution {
  root: Trie | null = null;
  board: string[][] = [];
  constructor() {
    this.root = new Trie();
  }
  insertWord(word: string) {
    let node: Trie = this.root!;
    for (let i = 0; i < word.length; i++) {
      const ch = word[i]!.charCodeAt(0);
      const idx = ch - A;
      if (!node.children[idx]) node.children[idx] = new Trie();
      node = node.children[idx];
    }
    node.word = word;
  }

  hasChild(node: Trie): boolean {
    for (const n of node.children) if (n) return true;
    return false;
  }

  dfs(r: number, c: number, node: Trie): string[] {
    let res: string[] = [];
    if (r < 0 || c < 0 || r >= this.board.length || c >= this.board[0]!.length || this.board[r]![c] === '#') return res;
    let ch = this.board[r]![c];
    let idx = ch.charCodeAt(0) - A;
    let cur = node.children[idx];
    if (!cur) return res;
    if (cur.word != "") {
      res.push(cur.word);
      cur.word = "";
    }
    this.board[r]![c] = '#';
    res = res.concat(this.dfs(r + 1, c, cur));
    res = res.concat(this.dfs(r, c + 1, cur));
    res = res.concat(this.dfs(r - 1, c, cur));
    res = res.concat(this.dfs(r, c - 1, cur));
    this.board[r]![c] = ch;
    if (cur.word.length === 0 && !this.hasChild(cur)) node.children[idx] = null;
    return res;
  }

  findWords(board: string[][], words: string[]): string[] {
    if (!board.length || !board[0]?.length || !words.length) return [];
    this.board = board;
    for (const word of words) {
      this.insertWord(word);
    }
    let res: string[] = [];
    for (let r = 0; r < board.length; r++)
      for (let c = 0; c < board[0].length; c++)
        res = res.concat(this.dfs(r, c, this.root!));
    return res;
  }
}
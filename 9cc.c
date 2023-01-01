#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
  TK_RESERVED,  // 記号
  TK_NUM,       // 整数トークン
  TK_EOF,       // 入力の終端を表すトークン
} TokenKind;

typedef struct Token Token;

struct Token {
  TokenKind kind;  // トークンの型
  Token *next;     // 次の入力トークン
  int val;         // kindがTK_NUMの場合、その数値
  char *str;       // トークン文字列
};

// 現在着目しているトークン
Token *token;

// エラーを報告するための関数
// printfと同じ引数を取る
void error(char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  vfprintf(stderr, fmt, ap);
  fprintf(stderr, fmt, ap);
  fprintf(stderr, "\n");
  exit(1);
}

// 次のトークンが期待する記号のときならトークンを一つ進めて真を返す
// そうでなければ偽を返す
bool consume(char op) {
  if (token->kind != TK_RESERVED || token->str[0] != op) {
    return false;
  }
  token = token->next;
  return true;
}

// 次のトークンが期待する記号ならトークンを一つ読み進める
// そうでなければエラーを返す
void expect(char op) {
  if (token->kind != TK_RESERVED || token->str[0] != op) {
    error("'%c'ではありません", op);
  }
  token = token->next;
}

// 次のトークンが数値ならトークンを一つ勧めてその数値を返す
// そうでなければエラーを返す
int expect_number() {
  if (token->kind != TK_NUM) {
    error("数ではありません");
  }
  int val = token->val;
  token = token->next;
  return val;
}

// 終端かどうか
bool at_eof() { return token->kind == TK_EOF; }

// 新しいトークンの生成，curにつなげる
Token *new_token(TokenKind kind, Token *cur, char *str) {
  Token *tok = calloc(1, sizeof(Token));
  tok->kind = kind;
  tok->str = str;
  cur->next = tok;
  return tok;
}

Token *tokenize(char *p) {
  Token head;
  head.next = NULL;
  Token *cur = &head;

  while (*p) {
    if (isspace(*p)) {
      p++;
      continue;
    }

    if (*p == '+' || *p == '-') {
      cur = new_token(TK_RESERVED, cur, p++);
      continue;
    }

    if (isdigit(*p)) {
      cur = new_token(TK_NUM, cur, p);
      cur->val = strtol(p, &p, 10);
      continue;
    }

    error("トーク内図できません");
  }

  new_token(TK_EOF, cur, p);
  return head.next;
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    fprintf(stderr, "引数の個数が正しくありません");
    return 1;
  }

  token = tokenize(argv[1]);

  // アセンブリの前半部分を出力
  printf(".intel_syntax noprefix\n");
  printf(".globl main\n");
  printf("main:\n");

  // 式の始まりは必ず数なのでチェック
  // mov命令を出力

  printf("  mov rax, %d\n", expect_number());

  // + <数>または - <数>というトークンの並びを消費してアセンブリを出力
  while (!at_eof()) {
    if (consume('+')) {
      printf("  add rax, %d\n", expect_number());
      continue;
    }

    expect('-');
    printf("  sub rax, %d\n", expect_number());

    printf("  ret\n");
    return 0;
  }
}
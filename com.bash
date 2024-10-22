#!/usr/bin/env bash
# 生成さたirファイルを指定の名前で実行可能ファイルとして生成する

# 引数がない場合は終了する
if [ $# -eq 0 ]; then
  echo "引数がありません。スクリプトを終了します。"
  exit 1
fi
# 引数の取得
ir_path=$1
ext_path=$2
ir_file_name=${ir_path%.*}


#echo ${ir_path} ${ext_path}


# コンパイル
cargo run --features "lexer,parser,decoder"
# オブジェクトファイルに変換
llc -relocation-model=pic -filetype=obj "${ir_path}"
# オブジェクトファイルをリンクして実行可能ファイルを生成
#g++ "${ir_file_name}.o" -o  "${ext_path}" -pie
g++  "${ir_file_name}.o"  -o  "${ext_path}" -pie

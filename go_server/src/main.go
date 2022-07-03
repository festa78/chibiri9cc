package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello, World in go\n")

	resp, err := http.Get("http://nginx-srv/compilerapi/")
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on compilerapi: %s\n", err))
		return
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on compilerapi: %s\n", err))
		return
	}

	fmt.Fprintf(w, string(body))
}

func main() {
	http.HandleFunc("/", handler) // ハンドラを登録してウェブページを表示させる
	http.ListenAndServe(":8080", nil)
}

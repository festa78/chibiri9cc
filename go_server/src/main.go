package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"net/http"
	"os/exec"
	"strings"
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

func assert(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	fmt.Fprintf(w, "assert handler is called\n")

	// Read and parse the request body.
	buf := new(bytes.Buffer)
	buf.ReadFrom(r.Body)
	s := buf.String()
	ss := strings.SplitN(strings.TrimLeft(s, " "), " ", 2)
	expected, input_code := ss[0], ss[1]

	resp, err := http.Post("http://nginx-srv/compilerapi/compile/", "text/plain", bytes.NewBufferString((input_code)))
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on compilerapi: %s\n", err))
		return
	}

	defer resp.Body.Close()
	compiled, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on compilerapi: %s\n", err))
		return
	}

	// Write down the assembly result.
	// ioutil.WriteFile("tmp.s", compiled, 0644)

	// Compile the assembly code.
	out_gcc, err := exec.Command("sh", "-c", fmt.Sprintf("echo '%s' | gcc -x assembler -static -o tmp -", compiled)).CombinedOutput()
	// err = exec.Command("bash", "-c", "gcc -static -o tmp tmp.s").Run()
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("out gcc: %s\n", out_gcc))
		fmt.Fprintf(w, fmt.Sprintf("fail on gcc: %s\n", err))
		return
	}

	out_exec, err := exec.Command("bash", "-c", "./tmp").CombinedOutput()
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("out exec: %s\n", out_exec))
		fmt.Fprintf(w, fmt.Sprintf("fail on execution: %s\n", err))
		return
	}

	if string(out_exec) != expected {
		fmt.Fprintf(w, fmt.Sprintf("expected: %s, actual: %s\n", expected, string(out_exec)))
	} else {
		fmt.Fprintf(w, "success\n")
	}
}

func main() {
	http.HandleFunc("/", handler)      // ハンドラを登録してウェブページを表示させる
	http.HandleFunc("/assert", assert) // ハンドラを登録してウェブページを表示させる
	http.ListenAndServe(":8080", nil)
}

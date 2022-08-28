import os

# diff {file} ../rCore-Tutorial-v3/{file}

if __name__ == '__main__':
    diff_dir = "../rCore-Tutorial-v3"
    # ends_with = [".rs",".py","Makefile",".S",".ld",".toml"]
    ends_with = ["linker-qemu.ld"]

    for (root, dirs, filenames) in os.walk(".", topdown=True):
        if "target" in root or ".git" in root:
            continue
            
        for filename in filenames:
            if not any(ele in filename for ele in ends_with):
                continue
                
            filepath = os.path.join(root, filename)
            diff_file = os.path.join(diff_dir, filepath)
            diff_cmd = f"diff {filepath} {diff_file}"
            print(diff_cmd)
            print(os.popen(diff_cmd).read())

    


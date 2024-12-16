import shakmaty_python_binding



def main():
    a=shakmaty_python_binding.MyChess()
    print(a )
    a.play('e2e3')
    print(a )


if __name__ == "__main__":
    main()
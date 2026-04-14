#include <stdio.h>
#include <string.h>

void mensagemBoaVinda(){

    printf("Boas vindas ao Proedufi!\n");
    printf("O seu app de gestão financeira\n");

}

void criarConta(){

    printf("Você escolheu criar uma conta\n");
    
    char usuarioDigitou[255];

    printf("[4] Para sair\n");

    char nome[63];
    char senha[255];

    scanf("%s", &usuarioDigitou);
    printf("%s", usuarioDigitou);
}



int main(void){

    mensagemBoaVinda();

    int escolhaUsuario = 0;

    printf("Escolha uma opção:\n");
    printf("[1] Fazer login\n");
    printf("[2] Criar conta\n");
    printf("[3] Continuar sem conta\n");
    scanf("%d", &escolhaUsuario);

    switch (escolhaUsuario){
        case 1:
            criarConta();
            break;
        case 2:
            printf("Crie sua conta");
            break;
        case 3:
            printf("Ok!");   
            break;
        default:
            printf("Escolha uma opção válida");
            break;
    }


    return 0;
}


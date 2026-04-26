#include <stdio.h>
#include <string.h>
#include <ctype.h>

void clearBuffer(){
	int x;
	while ((x = getchar()) != '\n' && x != EOF){

	}
}

void mensagemBoaVinda() {

    printf("Boas vindas ao Proedufi!\n");
    printf("O seu app de gestão financeira\n");

}

void limpaEspacoNoTexto(char *texto) {
    int textoOriginal = 0;
    int textoNovo = 0;
    int espaco_entre_letras = 0;

    //Conta todos os espaços iniciais
    while (isspace(texto[textoOriginal])) {
        textoOriginal++;
    }

    while (texto[textoOriginal] != '\0') {

        // So pode ter um espaco entre as letras, se houver mais de um, deve ser excluido
        if (isspace(texto[textoOriginal])) {

            if (espaco_entre_letras == 0) {
                texto[textoNovo] = ' ';
                textoNovo++;
                espaco_entre_letras = 1;
            }
        } else {
            texto[textoNovo] = texto[textoOriginal];
            textoNovo++;
            espaco_entre_letras = 0; //reseta a contagem de espacos entre as letras
        }

        textoOriginal++;
    }

    //Remove espaços do final
    if (textoNovo > 0 && isspace(texto[textoNovo - 1])) {
        textoNovo--;
    }

    //Coloca o indicador de que acabou o texto
    texto[textoNovo] = '\0';
    printf("%d\n", textoNovo);
}

void criarConta() {

    printf("Você escolheu criar uma conta\n");

    char usuarioDigitou[255];

    printf("[0] Para sair\n");

    char nome[63];
    char email[127];
    char senha[255];

    printf("Digite seu nome: ");

    clearBuffer();

    fgets(usuarioDigitou, 255, stdin);
    limpaEspacoNoTexto(usuarioDigitou);
    printf("%s\n", usuarioDigitou);


}



int main(void) {

    mensagemBoaVinda();

    int escolhaUsuario = 0;

    printf("Escolha uma opção:\n");
    printf("[1] Criar conta\n");
    printf("[2] Fazer login\n");
    printf("[3] Continuar sem conta\n");
    printf("[0] Sair!\n");
    scanf("%d", &escolhaUsuario);

    switch (escolhaUsuario) {
        case 1:
            criarConta();
            break;
        case 2:
            printf("Faça login na sua conta");
            break;
        case 3:
            printf("Ok!");
            break;
        case 0:
            printf("Até a próxima!");
            break;
        default:
            printf("Escolha uma opção válida");
            break;
    }


    return 0;
}

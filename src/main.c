#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>

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
    int espaco_entre_letras = false;

    //Conta todos os espaços iniciais
    while (isspace(texto[textoOriginal])) {
        textoOriginal++;
    }

    while (texto[textoOriginal] != '\0') {

        // So pode ter um espaco entre as letras, se houver mais de um, deve ser excluido
        if (isspace(texto[textoOriginal])) {

            if (espaco_entre_letras == false) {
                texto[textoNovo] = ' ';
                textoNovo++;
                espaco_entre_letras = true;
            }
        } else {
            texto[textoNovo] = texto[textoOriginal];
            textoNovo++;
            espaco_entre_letras = false; //reseta a contagem de espacos entre as letras
        }

        textoOriginal++;
    }

    //Remove espaços do final
    if (textoNovo > 0 && isspace(texto[textoNovo - 1])) {
        textoNovo--;
    }

    //Coloca o indicador de que acabou o texto
    texto[textoNovo] = '\0';
}

bool validaNome(char *nome){
    int tamanhoNome = strlen(nome);
    bool ehValido = false;

    for(int i = 0; i < tamanhoNome; i++){

        if(!isalpha(nome[i])){
            printf("O nome so pode ter letras\n");
            return ehValido;
        } else{
            ehValido = true;
        }
    }
    return ehValido;
}

bool validaEmail(char *email){
    int tamanhoEmail = strlen(email);
    bool temArroba = false;
    bool temPonto = false;
    bool ehValido = false;

    for(int i = 0; i < tamanhoEmail; i++){

        if(email[i] == ' '){
            printf("Nao pode ter espacos no email\n");
            return ehValido;
        }

        if(email[i] == '@'){
            if(temArroba == true){
                printf("O email nao pode ter dois  \"@\" (arroba)\n");
                return ehValido;
            }
            temArroba = true;
        }

        if(email[i] == '.'){
            temPonto = true;
        }

    }

    if(temArroba == false || temPonto == false){
        return ehValido;
    }

    if(email[0] != '@' && email[0] != '.' && email[tamanhoEmail] != '@' && email[tamanhoEmail - 1] != '.' ){
        ehValido = true;
    }

    return ehValido;
}


void criarConta() {

    printf("Você escolheu criar uma conta\n");
    printf("[0] Para voltar\n");

    char usuarioDigitou[255];

    char nome[60];
    char email[255];
    char senha[255];

    printf("Digite seu nome: ");
    clearBuffer(); // Pq aqui precisa e no outro fgets n?

    fgets(usuarioDigitou, 255, stdin);
    limpaEspacoNoTexto(usuarioDigitou);

    if(usuarioDigitou[0] == '0'){
        printf("Voce Voltou!\n\n");
        return;
    } else if(usuarioDigitou[0] == '\n'){
        printf("Digite o seu nome primeiro!\n");
    } else if(strlen(usuarioDigitou) > 60){
        printf("Nome muito extenso, digite seu nome real!\n");
    } else if(validaNome(usuarioDigitou) == false){
        printf("O nome so pode ter letras\n");
    } else{
        strcpy(nome, usuarioDigitou);
    }

    printf("Digite seu email: ");

    fgets(usuarioDigitou, 255, stdin);
    limpaEspacoNoTexto(usuarioDigitou); // eu chamei esta funcao para retirar os espacos iniciais e verificar se o digitou foi 0 ou n. Embora se houver espacos no email dara um erro mesmo assim

    if(usuarioDigitou[0] == '0'){
        printf("Voce Voltou!\n\n");
        return;
    } else if(usuarioDigitou[0] == '\n'){
        printf("Digite o seu email primeiro!\n");
    } else if(validaEmail(usuarioDigitou) == false){
        printf("Digite um email valido\n");
    } else{
        strcpy(email, usuarioDigitou);
    }

}



int main(void) {


    mensagemBoaVinda();

    int escolhaUsuario = 0;

    do {

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
                printf("Faça login na sua conta\n");
                break;
            case 3:
                printf("Ok!\n");
                break;
            case 0:
                printf("Até a próxima!\n");
                break;
            default:
                printf("Escolha uma opção válida\n");
                break;
        }
    }
    while(escolhaUsuario != 0);

    return 0;
}

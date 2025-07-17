# Rust Graphics Game Test
    
   Projeto feito enquanto não tenho chamados no trabalho. Tem como objetivo criar um pequeno jogo sendo o mais baixo nível possível.
   Atualmente conto com o uso do "Winit", como comunicador com a api de janelas do OS, e "Glutin", como gerenciador de contexto Opengl.
   Meu objetivo aqui é tentar entender como programar gráficos e como eles funcionam em um baixo nível, é claro que o projeto ainda possuí bibliotecas de abstração, mas o objetivo é tirar elas futuramente.

# DevLog

## 09/07/2025

   Hoje resolvi começar uma DevLog, para anotar o procedimento de desenvolvimento do projeto.
   Não fiz muita coisa, apenas consegui terminar um controle das vértices do triângulo vermelho, o que me deu certa dor de cabeça por não ter experiência com o sistema de variáveis do rust.
   O programa está assim:

   <img src = "assetsLog/primeira.png">

## 10/07/2025
    
   Brinquei um pouco tentando fazer um quadrado, primeiro tentei passar do glDrawArrays para o glDrawElements, fato que não consegui realizar, parte por minha ignorancia em opengl, parte por minha preguiça de realmente procurar um guia bem produzido, enfim, o resultado final foi esse:

   <img src = "assetsLog/quadrado.png">

## 14-15/07/2025
   
   Ok, fiz uma abstração no código, criando uma struct de objeto visual (Visual), e implementei um vetor de visuais no renderizador. Isso vai ajudar no futuro pois o código ficou menos engessado e mais dinâmico.

   <img src = "assetsLog/dois.png">

## 17/07/2025

   Fiz a movimentação 3D de uma pirâmide, mas não está 100% funcionando, existe algum erro

   1° Versão:
   <video src='https://media.discordapp.net/attachments/1105627560995459203/1395393568037802084/Glutin_triangle_gradient_example_press_Escape_to_exit_2025-07-17_10-14-59.mp4?ex=687a4907&is=6878f787&hm=e498f184d768a38df8939c08d1ae95ab0b00397803f7c2becb5eaabd16e2e1c0&format=webp&width=583&height=438' width=180/>
   
   2° Versão com alguns erros arrumados:
   *Olhar na pasta assetsLog/videos*

   3° Versão quase 100% certa
   *Olhar na pasta assetsLog/videos*

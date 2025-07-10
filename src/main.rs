use eframe::{NativeOptions, egui};
use egui::{Vec2};

/// A estrutura que representa o estado da nossa aplicação GUI.
struct TicTacToeApp {
    board: [char; 9],
    current_player: char,
    game_over: bool,
    winner: Option<char>,
    is_draw: bool,
}

impl Default for TicTacToeApp {
    /// Inicializa o estado padrão do jogo.
    fn default() -> Self {
        Self {
            board: [' '; 9],     // Tabuleiro vazio
            current_player: 'X', // 'X' começa o jogo
            game_over: false,
            winner: None,
            is_draw: false,
        }
    }
}

impl eframe::App for TicTacToeApp {
    /// O método `update` é chamado repetidamente para redesenhar a GUI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // CentralPanel ocupa todo o espaço disponível.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(20.0);

                // Título do jogo
                ui.heading("Jogo da Velha em Rust");
                ui.add_space(20.0);

                // Desenha o tabuleiro como uma grade de botões
                egui::Grid::new("tic_tac_toe_grid")
                    .spacing([15.0, 15.0]) // Espaçamento entre os elementos da grade
                    .show(ui, |ui| {
                        for i in 0..9 {
                            let cell_value = self.board[i];
                            let button_text = if cell_value == ' ' {
                                " ".to_string()
                            } else {
                                cell_value.to_string()
                            };

                            let button_response = ui.add_sized(
                                Vec2::new(60.0, 60.0), // Tamanho fixo para cada botão
                                egui::Button::new(button_text) // Passe o RichText aqui
                                    .wrap(), // 'strong(true)' agora faz parte do RichText, então removemos daqui
                            );

                            // Lógica de clique do mouse
                            if button_response.clicked() && !self.game_over {
                                // Verifica se a casa está vazia antes de fazer a jogada
                                if self.board[i] == ' ' {
                                    self.board[i] = self.current_player; // Faz a jogada

                                    // Verifica condições de vitória ou empate
                                    if check_winner(&self.board, self.current_player) {
                                        self.winner = Some(self.current_player);
                                        self.game_over = true;
                                    } else if self.board.iter().all(|&c| c != ' ') {
                                        self.is_draw = true;
                                        self.game_over = true;
                                    } else {
                                        // Alterna o jogador
                                        self.current_player =
                                            if self.current_player == 'X' { 'O' } else { 'X' };
                                    }
                                }
                            }

                            // A cada 3 botões, uma nova linha na grade
                            if (i + 1) % 3 == 0 {
                                ui.end_row();
                            }
                        }
                    });

                ui.add_space(20.0);

                // Exibe o status do jogo (vez do jogador, vencedor ou empate)
                if self.game_over {
                    if let Some(winner_char) = self.winner {
                        ui.heading(format!("🎉 Jogador {} venceu! 🎉", winner_char));
                    } else if self.is_draw {
                        ui.heading("🤝 É um Empate! 🤝");
                    }
                    ui.add_space(10.0);
                    // Botão para reiniciar o jogo
                    if ui.button("Jogar Novamente").clicked() {
                        *self = TicTacToeApp::default(); // Reinicia o estado do jogo
                    }
                } else {
                    ui.heading(format!("Vez do Jogador: {}", self.current_player));
                }
            });
        });
    }
}

/// Verifica se um jogador venceu.
/// Recebe o tabuleiro e o caractere do jogador ('X' ou 'O').
fn check_winner(board: &[char; 9], player: char) -> bool {
    // Todas as combinações possíveis de vitória (linhas, colunas, diagonais)
    let winning_combinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // Linhas
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // Colunas
        [0, 4, 8],
        [2, 4, 6], // Diagonais
    ];

    // Itera sobre as combinações e verifica se alguma delas é uma vitória para o jogador atual.
    winning_combinations
        .iter()
        .any(|&combo| combo.iter().all(|&i| board[i] == player))
}

/// Ponto de entrada principal da aplicação `eframe`.
fn main() -> eframe::Result<()> {
    // Opções nativas para a janela da aplicação (tamanho, título, etc.)
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 400.0])
            .with_min_inner_size([300.0, 400.0])
            .with_max_inner_size([300.0, 400.0])
            .with_resizable(false),
        ..Default::default()
    };

    // Executa a aplicação GUI
    eframe::run_native(
        "Jogo da Velha em Rust", // Título da janela
        native_options,
        Box::new(|_creation_context| {
            Ok::<Box<dyn eframe::App>, _>(Box::new(TicTacToeApp::default()))
        }),
    )
}

// Módulo para testes unitários (compilado apenas quando rodamos `cargo test`)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_winner() {
        let mut board = [' '; 9];
        board[0] = 'X';
        board[1] = 'X';
        board[2] = 'X';
        assert!(check_winner(&board, 'X'));
        assert!(!check_winner(&board, 'O'));
    }

    #[test]
    fn test_vertical_winner() {
        let mut board = [' '; 9];
        board[1] = 'O';
        board[4] = 'O';
        board[7] = 'O';
        assert!(check_winner(&board, 'O'));
        assert!(!check_winner(&board, 'X'));
    }

    #[test]
    fn test_diagonal_winner() {
        let mut board = [' '; 9];
        board[0] = 'X';
        board[4] = 'X';
        board[8] = 'X';
        assert!(check_winner(&board, 'X'));
    }

    #[test]
    fn test_no_winner() {
        let board = ['X', 'O', 'X', 'O', 'X', 'O', 'O', 'X', 'O'];
        assert!(!check_winner(&board, 'X'));
        assert!(!check_winner(&board, 'O'));
    }

    #[test]
    fn test_full_board_no_winner() {
        let board = ['X', 'O', 'X', 'O', 'X', 'O', 'O', 'X', 'O'];
        assert!(!check_winner(&board, 'X'));
        assert!(!check_winner(&board, 'O'));
        assert!(board.iter().all(|&c| c != ' ')); // Verifica se o tabuleiro está cheio para um empate
    }
}

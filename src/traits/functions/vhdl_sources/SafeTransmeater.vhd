

library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;


entity SafeTransmeater is
Generic(
    RECIVING_SIZE: integer := 79
);

  Port (
  CLK: in std_logic;
  
  TX: out std_logic;
  RX: in std_logic;
  
  reset: in std_logic;
 
  input_trigger: in std_logic := '0';
  
  input: in std_logic_vector(RECIVING_SIZE-1 downto 0);
  
  send_finished: out  std_logic := '0'
  
  );
end SafeTransmeater;

architecture Behavioral of SafeTransmeater is

Type StateType is (START,BEGIN_LOOP,SEND_DATA,SEND_DATA_1_1,SEND_DATA2,SENDING_CHECKSUM,SEND_CHECKSUM,READ_ACK,ACK,NACK,FINISH);

-- point to the first data to insert in the output that is not been checksummed yet
signal safe_cursor: unsigned(31 downto 0) := to_unsigned(0,32);
-- poit to the first data to insert in the output, evem if it is not checksumed
signal temp_cursor: unsigned(31 downto 0) := to_unsigned(0,32);

-- signal that keeps track of how many byte he has recived
signal counter: unsigned(3 downto 0) := "0000";

signal checksum: unsigned(7 downto 0) := "00000000";


signal state: StateType := START;

-- datas recived from the line
signal uart_rx_data: std_logic_vector(7 downto 0);
-- trigger rhat say we recived new datas
signal recive_trigger: std_logic;

-- datas to send from the line
signal uart_tx_data: std_logic_vector(7 downto 0);
-- trigger to send data
signal send_trigger: std_logic;
-- the sender has finishd;
signal send_finishd:  std_logic;

signal uart_reset: std_logic := '1';

signal data_to_send_signal: std_logic_vector(RECIVING_SIZE-1 downto 0);

begin

reciver: entity work.UartRX port map(
    CLK => CLK,
    RX => RX,
    data_out => uart_rx_data,
    reset => uart_reset,
    data_ready => recive_trigger
);

transmirter: entity work.UartTX port map(
    CLK => CLK,
    UART_TX => TX,
    DATA => uart_tx_data,
    SEND => send_trigger,
    READY => send_finishd
);



process(CLK,reset) begin
    
    if reset = '0' then
        state <= START;
    end if;
    
    if rising_edge(CLK) then
    
    
        -- begin state: reset everything
        if state = START then
            
            safe_cursor <= to_unsigned(0,32);
            temp_cursor <= to_unsigned(0,32);
            send_finished <= '0';
            
            if input_trigger = '1' then
                data_to_send_signal <= input;
                state <= BEGIN_LOOP;
            end if;
            
        elsif state = BEGIN_LOOP then
            
            checksum <= "00000000";
            counter <= "0000";
            uart_tx_data <= "00000000";
            state <= SEND_DATA;
        
        elsif state = SEND_DATA then
                        if temp_cursor + 7 <= to_unsigned(RECIVING_SIZE - 1,32) then
                --put the data inside
                uart_tx_data(0) <= data_to_send_signal(to_integer(temp_cursor));
                uart_tx_data(1) <= data_to_send_signal(to_integer(temp_cursor+1));
                uart_tx_data(2) <= data_to_send_signal(to_integer(temp_cursor+2));
                uart_tx_data(3) <= data_to_send_signal(to_integer(temp_cursor+3));
                uart_tx_data(4) <= data_to_send_signal(to_integer(temp_cursor+4));
                uart_tx_data(5) <= data_to_send_signal(to_integer(temp_cursor+5));
                uart_tx_data(6) <= data_to_send_signal(to_integer(temp_cursor+6));
                uart_tx_data(7) <= data_to_send_signal(to_integer(temp_cursor+7));                
            else
                if temp_cursor  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(0) <= data_to_send_signal(to_integer(temp_cursor));
                end if;
                if temp_cursor+1  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(1) <= data_to_send_signal(to_integer(temp_cursor+1));
                end if;
                if temp_cursor+2  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(2) <= data_to_send_signal(to_integer(temp_cursor+2));
                end if;
                if temp_cursor+3  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(3) <= data_to_send_signal(to_integer(temp_cursor+3));
                end if;
                if temp_cursor+4  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(4) <= data_to_send_signal(to_integer(temp_cursor+4));
                end if;
                if temp_cursor+5  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(5) <= data_to_send_signal(to_integer(temp_cursor+5));
                end if;
                if temp_cursor+6  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(6) <= data_to_send_signal(to_integer(temp_cursor+6));
                end if;
                if temp_cursor+7  <= to_unsigned(RECIVING_SIZE - 1,32) then
                    uart_tx_data(7) <= data_to_send_signal(to_integer(temp_cursor+7));
                end if;
            
            end if;
            
            temp_cursor <= temp_cursor+8;
            send_trigger <= '1';
            counter <= counter + 1;
            state <= SEND_DATA_1_1;
        
        -- WAIT FOR IT TO DOWN, OTHERWISE IT WOULD SKIP THE NEXT STATE
         elsif state = SEND_DATA_1_1 then
            if send_finishd = '0' then
                state <= SEND_DATA2;
            end if;
        elsif state = SEND_DATA2 then
            send_trigger <= '0';
            
            if send_finishd = '1' then
            
                checksum <= checksum + unsigned(uart_tx_data);
                
                -- finish sneding
                if counter = to_unsigned(9,4) or temp_cursor >= to_unsigned(RECIVING_SIZE,32) then
                   
                    state <= SENDING_CHECKSUM;
                else
                    state <= SEND_DATA;
                end if;

            end if;
        
        elsif state = SENDING_CHECKSUM then
            uart_tx_data <= std_logic_vector(checksum);
            send_trigger <= '1';
            state <= SEND_CHECKSUM;
        elsif state = SEND_CHECKSUM then
            send_trigger <= '0';
            
            if send_finishd = '1' then
                
                if recive_trigger = '1' then
                    uart_tx_data <= "00000000";
                    state <= READ_ACK;
                end if;
                
            end if;
        elsif state = READ_ACK then 
            
            
            case uart_rx_data is
                when "00000000" => state <= NACK;
                when "00000001" => state <= NACK;
                when "00000010" => state <= NACK;
                when "00000011" => state <= NACK;
                when "00000100" => state <= NACK;
                when "00000101" => state <= NACK;
                when "00000110" => state <= NACK;
                when "00000111" => state <= NACK;
                when "00001000" => state <= NACK;
                when "00001001" => state <= NACK;
                when "00001010" => state <= NACK;
                when "00001011" => state <= NACK;
                when "00001100" => state <= NACK;
                when "00001101" => state <= NACK;
                when "00001110" => state <= NACK;
                when "00001111" => state <= ACK;
                when "00010000" => state <= NACK;
                when "00010001" => state <= NACK;
                when "00010010" => state <= NACK;
                when "00010011" => state <= NACK;
                when "00010100" => state <= NACK;
                when "00010101" => state <= NACK;
                when "00010110" => state <= NACK;
                when "00010111" => state <= ACK;
                when "00011000" => state <= NACK;
                when "00011001" => state <= NACK;
                when "00011010" => state <= NACK;
                when "00011011" => state <= ACK;
                when "00011100" => state <= NACK;
                when "00011101" => state <= ACK;
                when "00011110" => state <= ACK;
                when "00011111" => state <= ACK;
                when "00100000" => state <= NACK;
                when "00100001" => state <= NACK;
                when "00100010" => state <= NACK;
                when "00100011" => state <= NACK;
                when "00100100" => state <= NACK;
                when "00100101" => state <= NACK;
                when "00100110" => state <= NACK;
                when "00100111" => state <= ACK;
                when "00101000" => state <= NACK;
                when "00101001" => state <= NACK;
                when "00101010" => state <= NACK;
                when "00101011" => state <= ACK;
                when "00101100" => state <= NACK;
                when "00101101" => state <= ACK;
                when "00101110" => state <= ACK;
                when "00101111" => state <= ACK;
                when "00110000" => state <= NACK;
                when "00110001" => state <= NACK;
                when "00110010" => state <= NACK;
                when "00110011" => state <= ACK;
                when "00110100" => state <= NACK;
                when "00110101" => state <= ACK;
                when "00110110" => state <= ACK;
                when "00110111" => state <= ACK;
                when "00111000" => state <= NACK;
                when "00111001" => state <= ACK;
                when "00111010" => state <= ACK;
                when "00111011" => state <= ACK;
                when "00111100" => state <= ACK;
                when "00111101" => state <= ACK;
                when "00111110" => state <= ACK;
                when "00111111" => state <= ACK;
                when "01000000" => state <= NACK;
                when "01000001" => state <= NACK;
                when "01000010" => state <= NACK;
                when "01000011" => state <= NACK;
                when "01000100" => state <= NACK;
                when "01000101" => state <= NACK;
                when "01000110" => state <= NACK;
                when "01000111" => state <= ACK;
                when "01001000" => state <= NACK;
                when "01001001" => state <= NACK;
                when "01001010" => state <= NACK;
                when "01001011" => state <= ACK;
                when "01001100" => state <= NACK;
                when "01001101" => state <= ACK;
                when "01001110" => state <= ACK;
                when "01001111" => state <= ACK;
                when "01010000" => state <= NACK;
                when "01010001" => state <= NACK;
                when "01010010" => state <= NACK;
                when "01010011" => state <= ACK;
                when "01010100" => state <= NACK;
                when "01010101" => state <= ACK;
                when "01010110" => state <= ACK;
                when "01010111" => state <= ACK;
                when "01011000" => state <= NACK;
                when "01011001" => state <= ACK;
                when "01011010" => state <= ACK;
                when "01011011" => state <= ACK;
                when "01011100" => state <= ACK;
                when "01011101" => state <= ACK;
                when "01011110" => state <= ACK;
                when "01011111" => state <= ACK;
                when "01100000" => state <= NACK;
                when "01100001" => state <= NACK;
                when "01100010" => state <= NACK;
                when "01100011" => state <= ACK;
                when "01100100" => state <= NACK;
                when "01100101" => state <= ACK;
                when "01100110" => state <= ACK;
                when "01100111" => state <= ACK;
                when "01101000" => state <= NACK;
                when "01101001" => state <= ACK;
                when "01101010" => state <= ACK;
                when "01101011" => state <= ACK;
                when "01101100" => state <= ACK;
                when "01101101" => state <= ACK;
                when "01101110" => state <= ACK;
                when "01101111" => state <= ACK;
                when "01110000" => state <= NACK;
                when "01110001" => state <= ACK;
                when "01110010" => state <= ACK;
                when "01110011" => state <= ACK;
                when "01110100" => state <= ACK;
                when "01110101" => state <= ACK;
                when "01110110" => state <= ACK;
                when "01110111" => state <= ACK;
                when "01111000" => state <= ACK;
                when "01111001" => state <= ACK;
                when "01111010" => state <= ACK;
                when "01111011" => state <= ACK;
                when "01111100" => state <= ACK;
                when "01111101" => state <= ACK;
                when "01111110" => state <= ACK;
                when "01111111" => state <= ACK;
                when "10000000" => state <= NACK;
                when "10000001" => state <= NACK;
                when "10000010" => state <= NACK;
                when "10000011" => state <= NACK;
                when "10000100" => state <= NACK;
                when "10000101" => state <= NACK;
                when "10000110" => state <= NACK;
                when "10000111" => state <= ACK;
                when "10001000" => state <= NACK;
                when "10001001" => state <= NACK;
                when "10001010" => state <= NACK;
                when "10001011" => state <= ACK;
                when "10001100" => state <= NACK;
                when "10001101" => state <= ACK;
                when "10001110" => state <= ACK;
                when "10001111" => state <= ACK;
                when "10010000" => state <= NACK;
                when "10010001" => state <= NACK;
                when "10010010" => state <= NACK;
                when "10010011" => state <= ACK;
                when "10010100" => state <= NACK;
                when "10010101" => state <= ACK;
                when "10010110" => state <= ACK;
                when "10010111" => state <= ACK;
                when "10011000" => state <= NACK;
                when "10011001" => state <= ACK;
                when "10011010" => state <= ACK;
                when "10011011" => state <= ACK;
                when "10011100" => state <= ACK;
                when "10011101" => state <= ACK;
                when "10011110" => state <= ACK;
                when "10011111" => state <= ACK;
                when "10100000" => state <= NACK;
                when "10100001" => state <= NACK;
                when "10100010" => state <= NACK;
                when "10100011" => state <= ACK;
                when "10100100" => state <= NACK;
                when "10100101" => state <= ACK;
                when "10100110" => state <= ACK;
                when "10100111" => state <= ACK;
                when "10101000" => state <= NACK;
                when "10101001" => state <= ACK;
                when "10101010" => state <= ACK;
                when "10101011" => state <= ACK;
                when "10101100" => state <= ACK;
                when "10101101" => state <= ACK;
                when "10101110" => state <= ACK;
                when "10101111" => state <= ACK;
                when "10110000" => state <= NACK;
                when "10110001" => state <= ACK;
                when "10110010" => state <= ACK;
                when "10110011" => state <= ACK;
                when "10110100" => state <= ACK;
                when "10110101" => state <= ACK;
                when "10110110" => state <= ACK;
                when "10110111" => state <= ACK;
                when "10111000" => state <= ACK;
                when "10111001" => state <= ACK;
                when "10111010" => state <= ACK;
                when "10111011" => state <= ACK;
                when "10111100" => state <= ACK;
                when "10111101" => state <= ACK;
                when "10111110" => state <= ACK;
                when "10111111" => state <= ACK;
                when "11000000" => state <= NACK;
                when "11000001" => state <= NACK;
                when "11000010" => state <= NACK;
                when "11000011" => state <= ACK;
                when "11000100" => state <= NACK;
                when "11000101" => state <= ACK;
                when "11000110" => state <= ACK;
                when "11000111" => state <= ACK;
                when "11001000" => state <= NACK;
                when "11001001" => state <= ACK;
                when "11001010" => state <= ACK;
                when "11001011" => state <= ACK;
                when "11001100" => state <= ACK;
                when "11001101" => state <= ACK;
                when "11001110" => state <= ACK;
                when "11001111" => state <= ACK;
                when "11010000" => state <= NACK;
                when "11010001" => state <= ACK;
                when "11010010" => state <= ACK;
                when "11010011" => state <= ACK;
                when "11010100" => state <= ACK;
                when "11010101" => state <= ACK;
                when "11010110" => state <= ACK;
                when "11010111" => state <= ACK;
                when "11011000" => state <= ACK;
                when "11011001" => state <= ACK;
                when "11011010" => state <= ACK;
                when "11011011" => state <= ACK;
                when "11011100" => state <= ACK;
                when "11011101" => state <= ACK;
                when "11011110" => state <= ACK;
                when "11011111" => state <= ACK;
                when "11100000" => state <= NACK;
                when "11100001" => state <= ACK;
                when "11100010" => state <= ACK;
                when "11100011" => state <= ACK;
                when "11100100" => state <= ACK;
                when "11100101" => state <= ACK;
                when "11100110" => state <= ACK;
                when "11100111" => state <= ACK;
                when "11101000" => state <= ACK;
                when "11101001" => state <= ACK;
                when "11101010" => state <= ACK;
                when "11101011" => state <= ACK;
                when "11101100" => state <= ACK;
                when "11101101" => state <= ACK;
                when "11101110" => state <= ACK;
                when "11101111" => state <= ACK;
                when "11110000" => state <= ACK;
                when "11110001" => state <= ACK;
                when "11110010" => state <= ACK;
                when "11110011" => state <= ACK;
                when "11110100" => state <= ACK;
                when "11110101" => state <= ACK;
                when "11110110" => state <= ACK;
                when "11110111" => state <= ACK;
                when "11111000" => state <= ACK;
                when "11111001" => state <= ACK;
                when "11111010" => state <= ACK;
                when "11111011" => state <= ACK;
                when "11111100" => state <= ACK;
                when "11111101" => state <= ACK;
                when "11111110" => state <= ACK;
                when others => state <= NACK;
            end case;
        

        elsif state = ACK then   
            
            if temp_cursor >= to_unsigned(RECIVING_SIZE,32) then
               state <= FINISH;
            else
                safe_cursor <= temp_cursor;
                state <= BEGIN_LOOP;
            end if;
            
        elsif state = NACK then
            
            temp_cursor <= safe_cursor;
            state <= BEGIN_LOOP;
            
        
        elsif state = FINISH then
            send_finished <= '1';
        end if;
    end if;
end process;



end Behavioral;

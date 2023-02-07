library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;


entity SafeReciver is
Generic(
    RECIVING_SIZE: integer := 79
);

  Port (
  CLK: in std_logic;
  
  TX: out std_logic;
  RX: in std_logic;
  
  reset: in std_logic;
  output_ready: out std_logic := '0';
  
  output: out std_logic_vector(RECIVING_SIZE-1 downto 0)
  
  );
end SafeReciver;

architecture Behavioral of SafeReciver is

Type StateType is (START,BEGIN_LOOP,READ_DATA,READ_CHECKSUM,CHECKSUM_WAS_WRONG,CHECKSUM_WAS_RIGHT,PRE_SEND_ACK,PRE_SEND_ACK2,SEND_ACK,FINISH);

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

signal uart_reset: std_logic := '1';

signal counter_to_send_ack: unsigned (31 downto 0);

signal finish_sending:  std_logic;

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
    READY => finish_sending
);



process(CLK,reset) begin
    
    if reset = '0' then
        state <= START;
    end if;
    
    if rising_edge(CLK) then
    
    
        -- begin state: reset everything
        if state = START then
         
            output_ready <= '0';
            --output <= (others => '0');
            safe_cursor <= to_unsigned(0,32);
            temp_cursor <= to_unsigned(0,32);
            checksum <= to_unsigned(0,8);
            counter <= to_unsigned(0,4);
            
            --go to the next state
            state <= BEGIN_LOOP;
            
            uart_reset <= '0';
        
        elsif state = BEGIN_LOOP then
            
            -- reset stuff
            checksum <= to_unsigned(0,8);
            counter <= to_unsigned(0,4);
            
            uart_reset <= '1';
            
            --go to the next state
            state <= READ_DATA;
        
        elsif state = READ_DATA then
            
            -- putting value in the array
            if recive_trigger = '1' then
                if temp_cursor+7 < to_unsigned(RECIVING_SIZE,32) then
                    --output(to_integer(temp_cursor+7) downto to_integer(temp_cursor)) <= uart_rx_data;
                    output(to_integer(temp_cursor)) <= uart_rx_data(0);
                    output(to_integer(temp_cursor+1)) <= uart_rx_data(1);
                    output(to_integer(temp_cursor+2)) <= uart_rx_data(2);
                    output(to_integer(temp_cursor+3)) <= uart_rx_data(3);
                    output(to_integer(temp_cursor+4)) <= uart_rx_data(4);
                    output(to_integer(temp_cursor+5)) <= uart_rx_data(5);
                    output(to_integer(temp_cursor+6)) <= uart_rx_data(6);
                    output(to_integer(temp_cursor+7)) <= uart_rx_data(7);
                    
                     -- sthill 8 since it wont be updated for the next clock cicle
                    if counter = 8 or temp_cursor+7 = to_unsigned(RECIVING_SIZE-1,32) then
                        state <= READ_CHECKSUM;
                    else
                        state <= READ_DATA;
                    end if;
                else
                    if to_integer(temp_cursor) < RECIVING_SIZE then
                        output(to_integer(temp_cursor)) <= uart_rx_data(0);
                    end if;
                    if to_integer(temp_cursor+1) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+1)) <= uart_rx_data(1);
                    end if;
                    if to_integer(temp_cursor+2) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+2)) <= uart_rx_data(2);
                    end if;
                    if to_integer(temp_cursor+3) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+3)) <= uart_rx_data(3);
                    end if;
                    if to_integer(temp_cursor+4) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+4)) <= uart_rx_data(4);
                    end if;
                    if to_integer(temp_cursor+5) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+5)) <= uart_rx_data(5);
                    end if;
                    if to_integer(temp_cursor+6) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+6)) <= uart_rx_data(6);
                    end if;
                    if to_integer(temp_cursor+7) < RECIVING_SIZE then
                        output(to_integer(temp_cursor+7)) <= uart_rx_data(7);
                    end if;
                    --output(RECIVING_SIZE-1 downto to_integer(temp_cursor)) <= uart_rx_data;
                    state <= READ_CHECKSUM;
                end if;
                
                checksum <= checksum + unsigned(uart_rx_data);
                counter <= counter +1;
                
                temp_cursor <= temp_cursor + 8;
                

                
            end if;
            
        
        elsif state = READ_CHECKSUM then
            if recive_trigger = '1' then
                
                if checksum = unsigned(uart_rx_data) then
                    state <= CHECKSUM_WAS_RIGHT;
                else 
                    state <= CHECKSUM_WAS_WRONG;
                end if;
                
            end if;
        
        elsif state = CHECKSUM_WAS_RIGHT then 
            safe_cursor <= temp_cursor;
            
            -- send an ok
            uart_tx_data <= "11111100";
            
            state <= PRE_SEND_ACK;
            counter_to_send_ack <= to_unsigned(0,32);
            
        elsif state = CHECKSUM_WAS_WRONG then   
            temp_cursor <= safe_cursor;
            
            -- send an error
            uart_tx_data <= "00000000";
            
            state <= PRE_SEND_ACK;
            counter_to_send_ack <= to_unsigned(0,32);
        
        elsif state = PRE_SEND_ACK then
            counter_to_send_ack <= counter_to_send_ack+1;
            if counter_to_send_ack >  to_unsigned(200000,32) then
                send_trigger <= '1';
            end if;
            if counter_to_send_ack >  to_unsigned(200010,32) then
                state <= PRE_SEND_ACK2;
            end if;
        
         elsif state = PRE_SEND_ACK2 then
            send_trigger <= '0';
            
            if finish_sending = '1' then
                state <= SEND_ACK;
            end if;
        
        elsif state = SEND_ACK then
        
            
            if safe_cursor + 1 >= to_unsigned(RECIVING_SIZE,32) then
                state <= FINISH;
            else
                state <= BEGIN_LOOP;
            end if;
        
        elsif state = FINISH then
            output_ready <= '1';
        end if;
    end if;
end process;



end Behavioral;

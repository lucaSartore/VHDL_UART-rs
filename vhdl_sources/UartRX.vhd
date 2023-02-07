
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

-- reciver for the nexis 8 bit, 9600 boud rate, no parity
entity UartRX is
    
    Generic(
        -- how many rising edge of the clock has to pass for sampling the next bit
        CLOCK_EDGE_FOR_BIT: integer := 10416;
        -- how many rising edge of the clock has to pass from the beginning of the start bit to the sampling of the bit zero
        CLOCK_EDGE_FOR_STOP_BIT: integer := 15624
    );
    
    Port(
    CLK: in std_logic;
    RX: in std_logic;
    data_out: out std_logic_vector(7 downto 0) := "00000000";
    data_ready: out std_logic := '0';
    reset: in std_logic
    );
end UartRX;

architecture Behavioral of UartRX is

-- the state
TYPE StateType IS (WAITING, WB0, WB1, WB2, WB3, WB4, WB5, WB6, WB7,PRE_WAITING,PRE_WAITING2);

signal current_state: StateType:= WAITING;

signal impulse_counter: unsigned(31 downto 0) := TO_UNSIGNED(0,32);

signal data_out_sigal: std_logic_vector(7 downto 0) := "00000000";
signal data_ready_signal:  std_logic := '0';



begin

-- process that detects if we nead to start reciving
states_transicitions: process(CLK,reset) begin
    
    if reset = '0' then
        current_state <= WAITING;
    end if;    
    
    if rising_edge (CLK) then
        
        if current_state = WAITING then
            -- start bit is here!
            
            data_ready <= '0';
            data_ready_signal <= '0';
            
            if RX = '0' then
                impulse_counter <= TO_UNSIGNED(15624,32);
                current_state <= WB0;
            end if;
        elsif impulse_counter = TO_UNSIGNED(0,32) then
        
            if current_state = WB0 then
                -- bit 0 is here!
    
                    data_out_sigal(0) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB1;
    
            elsif current_state = WB1 then
    
                    data_out_sigal(1) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB2;
    
            elsif current_state = WB2 then
    
                    data_out_sigal(2) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB3;
    
            elsif current_state = WB3 then
    
                    data_out_sigal(3) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB4;
    
            elsif current_state = WB4 then
    
                    data_out_sigal(4) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB5;
    
            elsif current_state = WB5 then
    
                    data_out_sigal(5) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB6;
    
            elsif current_state = WB6 then
            
                    data_out_sigal(6) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WB7;
    
            elsif current_state = WB7 then
    
                    data_out_sigal(7) <= RX;
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT*3,32);
                    current_state <= PRE_WAITING;
                   
                    data_out <= data_out_sigal;
                    data_out(7) <= RX;

    
            elsif current_state = PRE_WAITING then
            
                    impulse_counter <= TO_UNSIGNED(CLOCK_EDGE_FOR_BIT,32);
                    current_state <= WAITING;
                    data_ready <= '1';
                    data_ready_signal <= '1';  
            end if;
        
        else
            impulse_counter <= impulse_counter-1;
        end if;
        
        
        
    end if;
end process;



end Behavioral;
